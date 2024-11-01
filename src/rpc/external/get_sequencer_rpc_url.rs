use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrl {
    address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlResponse {
    pub sequencer_rpc_url: (String, Option<(String, String)>),
}

impl GetSequencerRpcUrl {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let sequencer_rpc_url = SequencerNodeInfo::get(&parameter.address)
            .map(|node_info| {
                (
                    node_info.external_rpc_url().to_owned(),
                    node_info.cluster_rpc_url().to_owned(),
                )
            })
            .ok();
        let sequencer_rpc_url = (parameter.address.as_hex_string(), sequencer_rpc_url);

        Ok(GetSequencerRpcUrlResponse { sequencer_rpc_url })
    }
}
