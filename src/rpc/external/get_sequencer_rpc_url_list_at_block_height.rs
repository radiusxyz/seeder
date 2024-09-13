use std::sync::Arc;

use radius_sequencer_sdk::signature::Signature;
use tracing::info;

use crate::{address::Address, error::Error, rpc::prelude::*, state::AppState, types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetSequencerRpcUrlListAtBlockHeigthMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    block_height: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeight {
    message: GetSequencerRpcUrlListAtBlockHeigthMessage,
    signature: Signature,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeighResponse {
    pub rpc_url_list: Vec<(String, Option<String>)>,
    pub block_height: u64,
}

impl GetSequencerRpcUrlListAtBlockHeight {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list_at_block_height";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListAtBlockHeighResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencerRpcUrlListAtBlockHeight>()?;

        info!(
            "get_sequencer_rpc_url_list_for_rollup: {:?}",
            parameter.message.cluster_id
        );

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
                    .find(|&&address| sdk_address == address)
                    .ok_or(Error::UnRegisteredFromContract)?;
            }
            _ => {}
        }

        let publisher = context.get_publisher(&sequencing_key).await?;
        let sequencer_list = publisher
            .get_sequencer_list(
                &parameter.message.cluster_id,
                parameter.message.block_height,
            )
            .await?;

        let rpc_url_list: Vec<(String, Option<String>)> = sequencer_list
            .into_iter()
            .filter_map(|address| {
                let address = Address::from(address.to_vec());
                SequencerNodeInfoModel::get(&address)
                    .ok()
                    .map(|sequencer| (address.to_string(), sequencer.rpc_url))
            })
            .collect();

        Ok(GetSequencerRpcUrlListAtBlockHeighResponse {
            rpc_url_list,
            block_height: parameter.message.block_height,
        })
    }
}
