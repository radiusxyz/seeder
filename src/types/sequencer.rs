pub use serde::{Deserialize, Serialize};

use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfo {
    pub sequencer_address: Vec<u8>,
    pub rpc_url: Option<String>,
}

impl SequencerNodeInfo {
    pub fn new(sequencer_address: Vec<u8>, rpc_url: Option<String>) -> Self {
        Self {
            sequencer_address,
            rpc_url,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfoModel;

impl SequencerNodeInfoModel {
    pub const ID: &'static str = stringify!(SequencerModel);

    pub fn get(address: &[u8]) -> Result<SequencerNodeInfo, DbError> {
        let key = (Self::ID, address);
        database()?.get(&key)
    }

    pub fn get_mut(address: &[u8]) -> Result<Lock<SequencerNodeInfo>, DbError> {
        let key = (Self::ID, address);
        database()?.get_mut(&key)
    }

    pub fn put(sequencer_node_info: &SequencerNodeInfo) -> Result<(), DbError> {
        let key = (Self::ID, &sequencer_node_info.sequencer_address);
        database()?.put(&key, sequencer_node_info)
    }

    pub fn delete(address: &[u8]) -> Result<(), DbError> {
        let key = (Self::ID, address);
        database()?.delete(&key)
    }
}
