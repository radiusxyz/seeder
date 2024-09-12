use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfoModel;

impl RollupNodeInfoModel {
    pub const ID: &'static str = stringify!(RollupNodeInfoModel);

    pub fn put(rollup_node_info: &RollupNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, &rollup_node_info.rollup_address);
        kvstore()?.put(&key, rollup_node_info)
    }

    pub fn get(address: &str) -> Result<RollupNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut_or_default(address: &str) -> Result<Lock<RollupNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut_or_default(&key)
    }

    pub fn delete(address: &str) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
