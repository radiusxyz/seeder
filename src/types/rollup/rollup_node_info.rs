pub use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfo {
    pub rollup_address: String,
    pub rpc_url: Option<String>,
}

impl RollupNodeInfo {
    pub fn new(rollup_address: String, rpc_url: Option<String>) -> Self {
        Self {
            rollup_address,
            rpc_url,
        }
    }
}
