use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Model)]
#[kvstore(key(address: &Address))]
pub struct SequencerNodeInfo {
    address: Address,
    external_rpc_url: String,
    cluster_rpc_url: String,
}

impl SequencerNodeInfo {
    pub fn new(address: Address, external_rpc_url: String, cluster_rpc_url: String) -> Self {
        Self {
            address,
            external_rpc_url,
            cluster_rpc_url,
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn external_rpc_url(&self) -> &String {
        &self.external_rpc_url
    }

    pub fn cluster_rpc_url(&self) -> &String {
        &self.cluster_rpc_url
    }
}
