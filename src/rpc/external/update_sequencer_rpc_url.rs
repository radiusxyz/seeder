use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, state::AppState, types::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct UpdateSequencerRpcUrlMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    chain_type: ChainType,
    address: Vec<u8>,
    rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSequencerRpcUrl {
    message: UpdateSequencerRpcUrlMessage,
    signature: Signature,
}

impl UpdateSequencerRpcUrl {
    pub const METHOD_NAME: &'static str = "update_sequencer_rpc_url";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<UpdateSequencerRpcUrl>()?;

        // // verify siganture
        // parameter.signature.verify_signature(
        //     rpc::methods::serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.as_slice(),
        //     parameter.message.chain_type,
        // )?;

        let sequencing_key = (
            parameter.message.platform,
            parameter.message.service_provider,
        );

        let sequencing_info = SequencingInfosModel::get()?;
        let sequencing_info_payload = sequencing_info
            .sequencing_infos()
            .get(&sequencing_key)
            .ok_or(Error::FailedToGetSequencingInfo)?;

        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(_payload) => {
                let publisher = context.get_publisher(&sequencing_key).await?;

                let block_number = publisher.get_block_number().await?;
                let sequencer_list = publisher
                    .get_sequencer_list(&parameter.message.cluster_id, block_number)
                    .await?;

                // check if the sequencer is registered in the contract
                sequencer_list
                    .iter()
                    .find(|&address| address.as_slice() == parameter.message.address)
                    .ok_or(Error::UnRegisteredFromContract)?;
            }
            _ => {}
        }

        // health check
        health_check(parameter.message.rpc_url.as_str()).await?;

        // update rpc url
        SequencerNodeInfoModel::apply(&parameter.message.address, |sequencer_node_info| {
            sequencer_node_info.rpc_url = Some(parameter.message.rpc_url);
            Ok(())
        })?;

        Ok(())
    }
}
