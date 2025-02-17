use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcUrl {
    address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcUrlResponse {
    pub tx_orderer_rpc_url: (String, Option<(String, String)>),
}

impl RpcParameter<AppState> for GetTxOrdererRpcUrl {
    type Response = GetTxOrdererRpcUrlResponse;

    fn method() -> &'static str {
        "get_tx_orderer_rpc_url"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let tx_orderer_rpc_url = TxOrdererNodeInfo::get(&self.address)
            .map(|node_info| {
                (
                    node_info.external_rpc_url().to_owned(),
                    node_info.cluster_rpc_url().to_owned(),
                )
            })
            .ok();
        let tx_orderer_rpc_url = (self.address.as_hex_string(), tx_orderer_rpc_url);

        Ok(GetTxOrdererRpcUrlResponse { tx_orderer_rpc_url })
    }
}
