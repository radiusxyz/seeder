pub use serde::{Deserialize, Serialize};

use crate::types::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfo {
    pub rollup_address: String,
    pub rpc_url: Option<String>,
}

impl RollupNodeInfo {
    pub fn new(address: String, rpc_url: Option<String>) -> Self {
        Self {
            rollup_address: address,
            rpc_url,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupNodeInfoModel;

impl RollupNodeInfoModel {
    pub const ID: &'static str = stringify!(RollupNodeInfoModel);

    pub fn get(address: &str) -> Result<RollupNodeInfo, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get(&key)
    }

    pub fn get_mut(address: &str) -> Result<Lock<RollupNodeInfo>, KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.get_mut(&key)
    }

    pub fn apply(
        address: &str,
        f: impl FnOnce(&mut RollupNodeInfo) -> Result<(), KvStoreError>,
    ) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.apply(&key, |locked_value: &mut Lock<RollupNodeInfo>| {
            f(locked_value).unwrap()
        })?;

        Ok(())
    }

    pub fn put(rollup_node_info: &RollupNodeInfo) -> Result<(), KvStoreError> {
        let key = (Self::ID, &rollup_node_info.rollup_address);
        kvstore()?.put(&key, rollup_node_info)
    }

    pub fn delete(address: &str) -> Result<(), KvStoreError> {
        let key = (Self::ID, address);
        kvstore()?.delete(&key)
    }
}
