use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcUrlList {
    tx_orderer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxOrdererRpcInfo {
    pub address: String,
    pub external_rpc_url: Option<String>,
    pub cluster_rpc_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcUrlListResponse {
    pub tx_orderer_rpc_url_list: Vec<TxOrdererRpcInfo>,
}

impl RpcParameter<AppState> for GetTxOrdererRpcUrlList {
    type Response = GetTxOrdererRpcUrlListResponse;

    fn method() -> &'static str {
        "get_tx_orderer_rpc_url_list"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let tx_orderer_rpc_url_list: Vec<TxOrdererRpcInfo> = self
            .tx_orderer_address_list
            .into_iter()
            .map(|address| match TxOrdererNodeInfo::get(&address) {
                Ok(node_info) => TxOrdererRpcInfo {
                    address: address.as_hex_string(),
                    external_rpc_url: Some(node_info.external_rpc_url().to_owned()),
                    cluster_rpc_url: Some(node_info.cluster_rpc_url().to_owned()),
                },
                Err(_) => TxOrdererRpcInfo {
                    address: address.as_hex_string(),
                    external_rpc_url: None,
                    cluster_rpc_url: None,
                },
            })
            .collect();

        Ok(GetTxOrdererRpcUrlListResponse {
            tx_orderer_rpc_url_list,
        })
    }
}
