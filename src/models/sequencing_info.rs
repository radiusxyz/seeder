use crate::{
    models::prelude::*,
    sequencer_types::prelude::{SequencingInfo, SequencingInfoKey, SequencingInfoPayload},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequencingInfoModel {
    sequencing_info: SequencingInfo,
}

impl SequencingInfoModel {
    pub fn new(
        sequencing_info_key: SequencingInfoKey,
        sequencing_info_payload: SequencingInfoPayload,
    ) -> Self {
        Self {
            sequencing_info: SequencingInfo::new(sequencing_info_key, sequencing_info_payload),
        }
    }

    pub fn sequencing_info_key(self) -> SequencingInfoKey {
        self.sequencing_info.sequencing_info_key()
    }

    pub fn sequencing_info_payload(&self) -> &SequencingInfoPayload {
        self.sequencing_info.sequencing_info_payload()
    }
}

impl SequencingInfoModel {
    pub const ID: &'static str = stringify!(SequencingInfoModel);

    pub fn get(sequencing_info_key: SequencingInfoKey) -> Result<Self, DbError> {
        let key = (Self::ID, sequencing_info_key);
        database()?.get(&key)
    }

    pub fn get_mut(sequencing_info_key: SequencingInfoKey) -> Result<Lock<'static, Self>, DbError> {
        let key = (Self::ID, sequencing_info_key);
        database()?.get_mut(&key)
    }

    pub fn put(&self, sequencing_info_key: SequencingInfoKey) -> Result<(), DbError> {
        let key = (Self::ID, sequencing_info_key);
        database()?.put(&key, self)
    }
}
