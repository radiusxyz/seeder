use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    signature::Signature,
};
use serde::{Deserialize, Serialize};

use crate::{
    address::SequencerAddress, error::Error, state::AppState, types::prelude::*, util::health_check,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: SequencerAddress,
    rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSequencer {
    message: RegisterSequencerMessage,
    signature: Signature,
}

impl RegisterSequencer {
    pub const METHOD_NAME: &'static str = "register_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<RegisterSequencer>()?;

        // // verify siganture
        // parameter.signature.verify_signature(
        //     crate::rpc::methods::serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.to_vec().as_slice(),
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
                    .find(|&&address| parameter.message.address == address)
                    .ok_or(Error::UnRegisteredFromContract)?;
            }
            _ => {}
        }

        // health check
        health_check(parameter.message.rpc_url.as_str()).await?;

        let mut sequencer_node_info =
            SequencerNodeInfoModel::get_mut_or_default(&parameter.message.address)?;
        sequencer_node_info.sequencer_address = parameter.message.address.to_string();
        sequencer_node_info.rpc_url = Some(parameter.message.rpc_url);
        sequencer_node_info.update()?;

        Ok(())
    }
}
