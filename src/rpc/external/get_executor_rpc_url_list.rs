use std::sync::Arc;

use crate::{address::Address, rpc::prelude::*, state::AppState, types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlList {
    executor_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetExecutorRpcUrlListResponse {
    pub executor_rpc_url_list: Vec<(Address, Option<String>)>,
}

impl GetExecutorRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_executor_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetExecutorRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<GetExecutorRpcUrlList>()?;

        let executor_rpc_url_list: Vec<(Address, Option<String>)> = parameter
            .executor_address_list
            .into_iter()
            .filter_map(|address| {
                RollupNodeInfoModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .collect();

        Ok(GetExecutorRpcUrlListResponse {
            executor_rpc_url_list,
        })
    }
}
