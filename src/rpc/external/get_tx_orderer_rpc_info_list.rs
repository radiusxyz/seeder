use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcInfoList {
    tx_orderer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcInfoListResponse {
    pub tx_orderer_rpc_info_list: Vec<ExecutorRpcInfo>,
}

impl RpcParameter<AppState> for GetTxOrdererRpcInfoList {
    type Response = GetTxOrdererRpcInfoListResponse;

    fn method() -> &'static str {
        "get_tx_orderer_rpc_info_list"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let tx_orderer_rpc_info_list: Vec<ExecutorRpcInfo> = self
            .tx_orderer_address_list
            .into_iter()
            .filter_map(|tx_orderer_address| ExecutorRpcInfo::get(&tx_orderer_address).ok())
            .collect();

        Ok(GetTxOrdererRpcInfoListResponse {
            tx_orderer_rpc_info_list: tx_orderer_rpc_info_list,
        })
    }
}
