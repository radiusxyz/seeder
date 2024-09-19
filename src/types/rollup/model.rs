use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfoModel;

impl RollupNodeInfoModel {
    pub const ID: &'static str = stringify!(RollupNodeInfoModel);

    pub fn put(address: &Address, rollup_node_info: &RollupNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.put(&key, rollup_node_info)
    }

    pub fn get(address: &Address) -> Result<RollupNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut(address: &Address) -> Result<Lock<RollupNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut(&key)
    }

    pub fn delete(address: &Address) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
