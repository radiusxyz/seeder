use crate::types::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequencingInfosModel;

impl SequencingInfosModel {
    pub const ID: &'static str = stringify!(SequencingInfosModel);

    pub fn get() -> Result<SequencingInfos, KvStoreError> {
        let key = Self::ID;

        kvstore()?.get(&key)
    }

    pub fn get_mut() -> Result<Lock<'static, SequencingInfos>, KvStoreError> {
        let key = Self::ID;

        kvstore()?.get_mut(&key)
    }

    pub fn get_mut_or_default() -> Result<Lock<'static, SequencingInfos>, KvStoreError> {
        let key = Self::ID;

        kvstore()?.get_mut_or_default(&key)
    }
}
