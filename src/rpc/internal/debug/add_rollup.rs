use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    signature::Signature,
};
use serde::{Deserialize, Serialize};

use crate::{
    address::Address, error::Error, state::AppState, types::prelude::*, util::health_check,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AddRollupMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddRollup {
    message: AddRollupMessage,
    signature: Signature,
}

impl AddRollup {
    pub const METHOD_NAME: &'static str = "add_rollup";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<AddRollup>()?;

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

        //
        let sdk_address = parameter
            .message
            .address
            .to_sdk_address(to_sdk_platform(parameter.message.platform))?;

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
                    .find(|&address| sdk_address == address);
            }
            _ => {}
        }

        // health check
        health_check(parameter.message.rpc_url.as_str()).await?;

        match RollupNodeInfoModel::get_mut(&parameter.message.address) {
            Ok(mut rollup_node_info) => {
                rollup_node_info.rollup_address = parameter.message.address.clone();
                rollup_node_info.rpc_url = Some(parameter.message.rpc_url);
                rollup_node_info.update()?;
            }
            Err(error) => {
                if error.is_none_type() {
                    let rollup_node_info = RollupNodeInfo::new(
                        parameter.message.address.clone(),
                        Some(parameter.message.rpc_url),
                    );
                    RollupNodeInfoModel::put(&rollup_node_info)?;
                } else {
                    return Err(error.into());
                }
            }
        }

        Ok(())
    }
}
