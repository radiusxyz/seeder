use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrl {
    address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlResponse {
    pub sequencer_rpc_url: (String, Option<(String, String)>),
}

impl RpcParameter<AppState> for GetSequencerRpcUrl {
    type Response = GetSequencerRpcUrlResponse;

    fn method() -> &'static str {
        "get_sequencer_rpc_url"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let sequencer_rpc_url = SequencerNodeInfo::get(&self.address)
            .map(|node_info| {
                (
                    node_info.external_rpc_url().to_owned(),
                    node_info.cluster_rpc_url().to_owned(),
                )
            })
            .ok();
        let sequencer_rpc_url = (self.address.as_hex_string(), sequencer_rpc_url);

        Ok(GetSequencerRpcUrlResponse { sequencer_rpc_url })
    }
}
