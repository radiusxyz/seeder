use std::sync::Arc;

use radius_sequencer_sdk::signature::{ChainType, Signature};
use tracing::info;

use crate::{
    error::Error, models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*, state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetClusterInfoMessage {
    sequencer_address_list: Vec<Vec<u8>>,
    rollup_address_list: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterInfo {
    signature: Signature,
    message: GetClusterInfoMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterInfoResponse {
    pub sequencer_rpc_url_list: Vec<(Vec<u8>, Option<String>)>,
    pub rollup_rpc_url_list: Vec<(Vec<u8>, Option<String>)>,
}

impl GetClusterInfo {
    pub const METHOD_NAME: &'static str = "get_cluster_info";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetClusterInfoResponse, RpcError> {
        let parameter = parameter.parse::<GetClusterInfo>()?;

        let sequencer_rpc_url_list: Vec<(Vec<u8>, Option<String>)> = parameter
            .message
            .sequencer_address_list
            .into_iter()
            .filter_map(|address| {
                SequencerNodeInfoModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .collect();

        let rollup_rpc_url_list: Vec<(Vec<u8>, Option<String>)> = parameter
            .message
            .rollup_address_list
            .into_iter()
            .filter_map(|address| {
                RollupNodeInfoModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .collect();

        Ok(GetClusterInfoResponse {
            sequencer_rpc_url_list,
            rollup_rpc_url_list,
        })
    }
}
