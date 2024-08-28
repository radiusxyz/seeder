use std::{net::IpAddr, sync::Arc};

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::{publisher::Publisher, types::Address},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error, models::prelude::SequencerModel, rpc::methods::serialize_to_bincode,
    sequencer_types::prelude::*, util::health_check,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
    rpc_url: IpAddr,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Register {
    signature: Signature,
    message: RegisterMessage,
}

impl Register {
    pub const METHOD_NAME: &'static str = "register";

    pub async fn handler(parameter: RpcParameter, context: Arc<Publisher>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Register>()?;

        // verify siganture
        parameter.signature.verify_signature(
            serialize_to_bincode(&parameter.message)?.as_slice(),
            parameter.message.address.as_slice(),
            parameter.message.chain_type,
        )?;

        let block_number = context.get_block_number().await?;
        let sequencer_list = context
            .get_sequencer_list(&parameter.message.cluster_id, block_number)
            .await?;

        // check if the sequencer is registered
        sequencer_list
            .iter()
            .find(|&address| address.as_slice() == parameter.message.address)
            .ok_or(Error::UnRegisteredFromContract)?;

        // health check
        let rpc_url = IpAddress::from(parameter.message.rpc_url);
        health_check(rpc_url.as_ref()).await?;

        match SequencerModel::get(&parameter.message.address) {
            // TODO: change(tmp logic when already registered)
            Ok(sequencer) => {
                tracing::warn!("Already registered sequencer: {:?}", sequencer);

                let sequencer = SequencerModel::new(parameter.message.address, Some(rpc_url));

                sequencer.put()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    let sequencer = SequencerModel::new(parameter.message.address, Some(rpc_url));

                    sequencer.put()?;
                } else {
                    tracing::error!("Failed to add sequencer: {:?}", err);
                    return Err(err.into());
                }
            }
        };

        Ok(())
    }
}
