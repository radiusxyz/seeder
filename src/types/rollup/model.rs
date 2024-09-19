use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfoModel;

impl RollupNodeInfoModel {
    pub const ID: &'static str = stringify!(RollupNodeInfoModel);

    pub fn put(address: &Address, rollup_rpc_url: &String) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.put(&key, rollup_rpc_url)
    }

    pub fn get(address: &Address) -> Result<String, KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.get(&key)
    }

    pub fn get_mut(address: &Address) -> Result<Lock<String>, KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.get_mut(&key)
    }

    pub fn delete(address: &Address) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);

        kvstore()?.delete(&key)
    }
}
