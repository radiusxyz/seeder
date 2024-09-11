pub use serde::{Deserialize, Serialize};

use crate::types::prelude::*;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfoModel;

impl SequencerNodeInfoModel {
    pub const ID: &'static str = stringify!(SequencerModel);

    pub fn get(address: &str) -> Result<SequencerNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut(address: &str) -> Result<Lock<SequencerNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut(&key)
    }

    pub fn put(sequencer_node_info: &SequencerNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, &sequencer_node_info.sequencer_address);
        kvstore()?.put(&key, sequencer_node_info)
    }

    pub fn apply(
        address: &str,
        f: impl FnOnce(&mut SequencerNodeInfo) -> Result<(), KvStoreError>,
    ) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.apply(&key, |locked_value: &mut Lock<SequencerNodeInfo>| {
            f(locked_value).unwrap()
        })?;

        Ok(())
    }

    pub fn delete(address: &str) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
