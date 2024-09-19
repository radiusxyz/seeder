use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    sequencer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub sequencer_rpc_url_list: Vec<(String, Option<String>)>,
}

impl GetSequencerRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let sequencer_rpc_url_list: Vec<(String, Option<String>)> = parameter
            .sequencer_address_list
            .into_iter()
            .map(|address| {
                (
                    address.to_string(),
                    SequencerNodeInfoModel::get(&address).ok(),
                )
            })
            .collect();

        Ok(GetSequencerRpcUrlListResponse {
            sequencer_rpc_url_list,
        })
    }
}
