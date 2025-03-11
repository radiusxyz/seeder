use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcInfoList {
    executor_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcInfoListResponse {
    pub executor_rpc_info_list: Vec<ExecutorRpcInfo>,
}

impl RpcParameter<AppState> for GetExecutorRpcInfoList {
    type Response = GetExecutorRpcInfoListResponse;

    fn method() -> &'static str {
        "get_executor_rpc_info_list"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let executor_rpc_info_list: Vec<ExecutorRpcInfo> = self
            .executor_address_list
            .into_iter()
            .filter_map(|executor_address| ExecutorRpcInfo::get(&executor_address).ok())
            .collect();

        Ok(GetExecutorRpcInfoListResponse {
            executor_rpc_info_list,
        })
    }
}
