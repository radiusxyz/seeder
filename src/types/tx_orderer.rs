use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Model)]
#[kvstore(key(address: &Address))]
pub struct TxOrdererRpcInfo {
    tx_orderer_address: Address,
    external_rpc_url: String,
    cluster_rpc_url: String,
}

impl TxOrdererRpcInfo {
    pub fn new(
        tx_orderer_address: Address,
        external_rpc_url: String,
        cluster_rpc_url: String,
    ) -> Self {
        Self {
            tx_orderer_address,
            external_rpc_url,
            cluster_rpc_url,
        }
    }

    pub fn tx_orderer_address(&self) -> &Address {
        &self.tx_orderer_address
    }

    pub fn external_rpc_url(&self) -> &String {
        &self.external_rpc_url
    }

    pub fn cluster_rpc_url(&self) -> &String {
        &self.cluster_rpc_url
    }
}
