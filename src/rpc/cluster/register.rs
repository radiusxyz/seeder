use std::{net::IpAddr, sync::Arc};

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::types::Address,
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::prelude::{ClusterInfoModel, SequencerModel},
    rpc::methods::serialize_to_bincode,
    sequencer_types::prelude::*,
    state::AppState,
    util::health_check,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterMessage {
    address: Address,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_provider: ServiceProvider,
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

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Register>()?;

        // verify siganture
        parameter.signature.verify_signature(
            serialize_to_bincode(&parameter.message)?.as_slice(),
            parameter.message.address.as_slice(),
            parameter.message.chain_type,
        )?;

        let publisher = context.get_publisher(SequencingInfoKey::new(
            Platform::from(parameter.message.chain_type),
            parameter.message.sequencing_function_type,
            parameter.message.service_provider,
        ))?;

        let block_number = publisher.get_block_number().await?;
        let sequencer_list = publisher
            .get_sequencer_list(&parameter.message.cluster_id, block_number)
            .await?;

        // check if the sequencer is registered in the contract
        sequencer_list
            .iter()
            .find(|&address| address.as_slice() == parameter.message.address)
            .ok_or(Error::UnRegisteredFromContract)?;

        // health check
        let rpc_url = IpAddress::from(parameter.message.rpc_url);
        health_check(rpc_url.as_ref()).await?;

        // add sequencer to cluster info
        let mut cluster_info = ClusterInfoModel::get_mut(&parameter.message.cluster_id)?;
        cluster_info.add_sequencer(parameter.message.address, Some(rpc_url.clone()))?;
        cluster_info.update()?;

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
