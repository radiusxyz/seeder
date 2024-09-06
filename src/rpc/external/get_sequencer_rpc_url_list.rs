use std::sync::Arc;

use radius_sequencer_sdk::signature::{ChainType, Signature};
use tracing::info;

use crate::{
    error::Error, models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*, state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetSequencerRpcUrlListMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    sequencer_address_list: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    signature: Signature,
    message: GetSequencerRpcUrlListMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub rpc_url_list: Vec<(Vec<u8>, Option<String>)>,
}

impl GetSequencerRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencerRpcUrlList>()?;

        info!(
            "get_sequencer_rpc_url_list: {:?}",
            parameter.message.cluster_id
        );

        // // verify siganture
        // parameter.signature.verify_signature(
        //     rpc::methods::serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.as_slice(),
        //     parameter.message.chain_type,
        // )?;

        let sequencing_key = sequencing_key(
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

        let rpc_url_list: Vec<(Vec<u8>, Option<String>)> = parameter
            .message
            .sequencer_address_list
            .into_iter()
            .filter_map(|address| {
                SequencerModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .collect();

        Ok(GetSequencerRpcUrlListResponse { rpc_url_list })
    }
}
