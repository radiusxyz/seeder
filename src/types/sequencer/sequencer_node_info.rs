pub use serde::{Deserialize, Serialize};

use crate::address::Address;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfo {
    pub sequencer_address: Address,
    pub rpc_url: Option<String>,
}

impl SequencerNodeInfo {
    pub fn new(sequencer_address: Address, rpc_url: Option<String>) -> Self {
        Self {
            sequencer_address,
            rpc_url,
        }
    }
}
