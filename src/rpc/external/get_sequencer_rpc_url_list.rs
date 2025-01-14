use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    sequencer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerRpcInfo {
    pub address: String,
    pub external_rpc_url: Option<String>,
    pub cluster_rpc_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub sequencer_rpc_url_list: Vec<SequencerRpcInfo>,
}

impl RpcParameter<AppState> for GetSequencerRpcUrlList {
    type Response = GetSequencerRpcUrlListResponse;

    fn method() -> &'static str {
        "get_sequencer_rpc_url_list"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let sequencer_rpc_url_list: Vec<SequencerRpcInfo> = self
            .sequencer_address_list
            .into_iter()
            .map(|address| match SequencerNodeInfo::get(&address) {
                Ok(node_info) => SequencerRpcInfo {
                    address: address.as_hex_string(),
                    external_rpc_url: Some(node_info.external_rpc_url().to_owned()),
                    cluster_rpc_url: Some(node_info.cluster_rpc_url().to_owned()),
                },
                Err(_) => SequencerRpcInfo {
                    address: address.as_hex_string(),
                    external_rpc_url: None,
                    cluster_rpc_url: None,
                },
            })
            .collect();

        Ok(GetSequencerRpcUrlListResponse {
            sequencer_rpc_url_list,
        })
    }
}
