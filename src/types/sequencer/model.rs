use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfoModel;

impl SequencerNodeInfoModel {
    pub const ID: &'static str = stringify!(SequencerModel);

    pub fn put(sequencer_node_info: &SequencerNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, &sequencer_node_info.sequencer_address);
        kvstore()?.put(&key, sequencer_node_info)
    }

    pub fn get(address: &str) -> Result<SequencerNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut_or_default(address: &str) -> Result<Lock<SequencerNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut_or_default(&key)
    }

    pub fn delete(address: &str) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
