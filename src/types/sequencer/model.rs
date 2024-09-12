use crate::{address::Address, types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfoModel;

impl SequencerNodeInfoModel {
    pub const ID: &'static str = stringify!(SequencerModel);

    pub fn put(sequencer_node_info: &SequencerNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, &sequencer_node_info.sequencer_address);
        kvstore()?.put(&key, sequencer_node_info)
    }

    pub fn get(address: &Address) -> Result<SequencerNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut(address: &Address) -> Result<Lock<SequencerNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut(&key)
    }

    pub fn delete(address: &Address) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
