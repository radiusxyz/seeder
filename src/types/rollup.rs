use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Model)]
#[kvstore(key(address: &Address))]
pub struct RollupNodeInfo {
    address: Address,
    rpc_url: String,
}

impl RollupNodeInfo {
    pub fn new(address: Address, rpc_url: String) -> Self {
        Self { address, rpc_url }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn rpc_url(&self) -> &String {
        &self.rpc_url
    }

    pub fn into_rpc_url(self) -> String {
        self.rpc_url
    }
}
