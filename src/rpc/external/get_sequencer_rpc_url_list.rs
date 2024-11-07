use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    sequencer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerRpcInfo {
    pub address: String,
    pub external_rpc_url: String,
    pub cluster_rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub sequencer_rpc_url_list: Vec<SequencerRpcInfo>,
}

impl GetSequencerRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let sequencer_rpc_url_list: Vec<SequencerRpcInfo> = parameter
            .sequencer_address_list
            .into_iter()
            .filter_map(|address| {
                SequencerNodeInfo::get(&address)
                    .ok()
                    .map(|node_info| SequencerRpcInfo {
                        address: address.as_hex_string(),
                        external_rpc_url: node_info.external_rpc_url().to_owned(),
                        cluster_rpc_url: node_info.cluster_rpc_url().to_owned(),
                    })
            })
            .collect();

        Ok(GetSequencerRpcUrlListResponse {
            sequencer_rpc_url_list,
        })
    }
}
