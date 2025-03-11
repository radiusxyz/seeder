use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Model)]
#[kvstore(key(address: &Address))]
pub struct ExecutorRpcInfo {
    executor_address: Address,
    rpc_url: String,
}

impl ExecutorRpcInfo {
    pub fn new(executor_address: Address, rpc_url: String) -> Self {
        Self {
            executor_address,
            rpc_url,
        }
    }

    pub fn address(&self) -> &Address {
        &self.executor_address
    }

    pub fn rpc_url(&self) -> &String {
        &self.rpc_url
    }

    pub fn into_rpc_url(self) -> String {
        self.rpc_url
    }
}
