use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlList {
    executor_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlListResponse {
    pub executor_rpc_url_list: Vec<(String, Option<String>)>,
}

impl RpcParameter<AppState> for GetExecutorRpcUrlList {
    type Response = GetExecutorRpcUrlListResponse;

    fn method() -> &'static str {
        "get_executor_rpc_url_list"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let executor_rpc_url_list: Vec<(String, Option<String>)> = self
            .executor_address_list
            .into_iter()
            .map(|address| {
                (
                    address.as_hex_string(),
                    RollupNodeInfo::get(&address)
                        .ok()
                        .map(|node_info| node_info.into_rpc_url()),
                )
            })
            .collect();

        Ok(GetExecutorRpcUrlListResponse {
            executor_rpc_url_list,
        })
    }
}
