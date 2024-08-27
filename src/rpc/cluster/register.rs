use std::{fmt::Formatter, sync::Arc};

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::{publisher::Publisher, types::hex},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, models::prelude::SequencerModel, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    cluster_id: ClusterId,
    rpc_url: IpAddress,
}

impl std::fmt::Display for RegisterMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
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
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
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
            .ok_or(Error::UnRegistered)?;

        let address = Address::from(hex::encode(&parameter.message.address));

        match SequencerModel::get(&Address::from(hex::encode(&parameter.message.address))) {
            // TODO: change(tmp logic when already registered)
            Ok(sequencer) => {
                tracing::warn!("Already registered sequencer: {:?}", sequencer);

                let sequencer = SequencerModel::new(address, parameter.message.rpc_url.into());

                sequencer.put()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    let sequencer = SequencerModel::new(address, parameter.message.rpc_url.into());

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
