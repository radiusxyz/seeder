use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::types::Address,
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetSequencerRpcUrlListMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
    sequencer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    signature: Signature,
    message: GetSequencerRpcUrlListMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub rpc_url_list: Vec<(Address, Option<IpAddress>)>,
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
        //     serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.as_slice(),
        //     parameter.message.chain_type,
        // )?;
        let sequencer_list = parameter.message.sequencer_address_list;

        let rpc_url_list: Vec<(Address, Option<IpAddress>)> = sequencer_list
            .into_iter()
            .map(|address| {
                SequencerModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .flatten()
            .collect();

        Ok(GetSequencerRpcUrlListResponse { rpc_url_list })
    }
}
