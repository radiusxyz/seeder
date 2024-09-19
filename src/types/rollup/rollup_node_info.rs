use radius_sequencer_sdk::signature::Address;
pub use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfo {
    pub rollup_address: Address,
    pub rpc_url: Option<String>,
}

impl RollupNodeInfo {
    pub fn new(rollup_address: Address, rpc_url: Option<String>) -> Self {
        Self {
            rollup_address,
            rpc_url,
        }
    }
}
