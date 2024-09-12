pub use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfo {
    pub rollup_address: String,
    pub rpc_url: Option<String>,
}

impl RollupNodeInfo {
    pub fn new(address: String, rpc_url: Option<String>) -> Self {
        Self {
            rollup_address: address,
            rpc_url,
        }
    }
}
