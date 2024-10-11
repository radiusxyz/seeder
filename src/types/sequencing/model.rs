use super::prelude::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SequencingInfoListModel;

impl SequencingInfoListModel {
    const ID: &'static str = stringify!(SequencingInfoListModel);

    pub fn get() -> Result<SequencingInfoList, KvStoreError> {
        let key = &(Self::ID);

        kvstore()?.get(key)
    }

    pub fn get_or_default() -> Result<SequencingInfoList, KvStoreError> {
        let key = &(Self::ID);

        kvstore()?.get_or_default(key)
    }

    pub fn get_mut_or_default() -> Result<Lock<'static, SequencingInfoList>, KvStoreError> {
        let key = &(Self::ID);

        kvstore()?.get_mut_or_default(key)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencingInfoPayloadModel;

impl SequencingInfoPayloadModel {
    const ID: &'static str = stringify!(SequencingInfoPayloadModel);

    pub fn put(
        platform: Platform,
        service_provider: ServiceProvider,
        value: &SequencingInfoPayload,
    ) -> Result<(), KvStoreError> {
        let key = &(Self::ID, platform, service_provider);

        kvstore()?.put(key, value)
    }

    pub fn get(
        platform: Platform,
        service_provider: ServiceProvider,
    ) -> Result<SequencingInfoPayload, KvStoreError> {
        let key = &(Self::ID, platform, service_provider);

        kvstore()?.get(key)
    }
}
