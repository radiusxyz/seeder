use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcUrl {
    tx_orderer_address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTxOrdererRpcInfoResponse {
    pub tx_orderer_rpc_info: TxOrdererRpcInfo,
}

impl RpcParameter<AppState> for GetTxOrdererRpcUrl {
    type Response = GetTxOrdererRpcInfoResponse;

    fn method() -> &'static str {
        "get_tx_orderer_rpc_info"
    }

    // self.tx_orderer_address.as_hex_string()
    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let tx_orderer_rpc_info = TxOrdererRpcInfo::get(&self.tx_orderer_address)?;

        Ok(GetTxOrdererRpcInfoResponse {
            tx_orderer_rpc_info,
        })
    }
}
