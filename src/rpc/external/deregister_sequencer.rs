use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::types::Address,
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error, models::prelude::SequencerModel, rpc::methods::serialize_to_bincode,
    sequencer_types::prelude::*, state::AppState,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeregisterSequencerMessage {
    address: Address,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_provider: ServiceProvider,
    cluster_id: ClusterId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterSequencer {
    signature: Signature,
    message: DeregisterSequencerMessage,
}

impl DeregisterSequencer {
    pub const METHOD_NAME: &'static str = "deregister_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<DeregisterSequencer>()?;

        let sequencing_info_key = SequencingInfoKey::new(
            Platform::from(parameter.message.chain_type),
            parameter.message.sequencing_function_type,
            parameter.message.service_provider,
        );

        // Todo: check if the sequencer is registered in the contract
        if matches!(
            SequencingCondition::from(sequencing_info_key),
            SequencingCondition::EthereumLivenessRadius
        ) {
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

            // check if the sequencer is deregistered from the contract
            if sequencer_list.contains(&parameter.message.address) {
                return Err(Error::NotDeregisteredFromContract.into());
            }
        }

        // Todo: remove cluster info
        // // remove sequencer from cluster info
        // let mut cluster_info = ClusterInfoModel::get_mut(&parameter.message.cluster_id)?;
        // cluster_info.remove_sequencer(&parameter.message.address)?;
        // cluster_info.update()?;

        // remove sequencer model
        match SequencerModel::get(&parameter.message.address) {
            Ok(sequencer) => {
                sequencer.delete()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    tracing::warn!("Already deregistered sequencer");
                } else {
                    return Err(err.into());
                }
            }
        }

        // Todo: remove cluster info
        // // remove sequencer from app state
        // context
        //     .get_cluster_info(&parameter.message.cluster_id)?
        //     .remove_sequencer(&parameter.message.address)?;

        Ok(())
    }
}
