pub use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfo {
    pub sequencer_address: String,
    pub rpc_url: Option<String>,
}

impl SequencerNodeInfo {
    pub fn new(sequencer_address: String, rpc_url: Option<String>) -> Self {
        Self {
            sequencer_address,
            rpc_url,
        }
    }
}
