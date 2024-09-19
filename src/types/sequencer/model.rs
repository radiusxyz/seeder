use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerNodeInfoModel;

impl SequencerNodeInfoModel {
    pub const ID: &'static str = stringify!(SequencerNodeInfoModel);

    pub fn put(address: &Address, sequencer_rpc_url: &String) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.put(&key, sequencer_rpc_url)
    }

    pub fn get(address: &Address) -> Result<String, KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.get(&key)
    }

    pub fn delete(address: &Address) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.delete(&key)
    }
}
