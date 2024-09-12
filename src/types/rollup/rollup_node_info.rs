pub use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfo {
    pub rollup_address: Vec<u8>,
    pub rpc_url: Option<String>,
}

impl RollupNodeInfo {
    pub fn new(address: Vec<u8>, rpc_url: Option<String>) -> Self {
        Self {
            rollup_address: address,
            rpc_url,
        }
    }
}
