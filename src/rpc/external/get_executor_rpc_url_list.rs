use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlList {
    executor_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlListResponse {
    pub executor_rpc_url_list: Vec<(String, Option<String>)>,
}

impl GetExecutorRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_executor_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetExecutorRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let executor_rpc_url_list: Vec<(String, Option<String>)> = parameter
            .executor_address_list
            .into_iter()
            .map(|address| (address.to_string(), RollupNodeInfoModel::get(&address).ok()))
            .collect();

        Ok(GetExecutorRpcUrlListResponse {
            executor_rpc_url_list,
        })
    }
}
