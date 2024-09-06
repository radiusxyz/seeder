use std::collections::BTreeMap;

use crate::{models::prelude::*, sequencer_types::prelude::SequencingInfoPayload};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SequencingInfosModel {
    sequencing_infos: BTreeMap<String, SequencingInfoPayload>,
}

impl SequencingInfosModel {
    pub fn new(sequencing_infos: BTreeMap<String, SequencingInfoPayload>) -> Self {
        Self { sequencing_infos }
    }

    pub fn sequencing_infos(&self) -> &BTreeMap<String, SequencingInfoPayload> {
        &self.sequencing_infos
    }

    pub fn insert(&mut self, key: String, value: SequencingInfoPayload) {
        self.sequencing_infos.insert(key, value);
    }
}

impl SequencingInfosModel {
    pub const ID: &'static str = stringify!(SequencingInfosModel);

    pub fn get() -> Result<Self, DbError> {
        let key = Self::ID;
        database()?.get(&key)
    }

    pub fn get_or_default() -> Result<Self, DbError> {
        let key = Self::ID;
        database()?.get_or_default(&key)
    }

    pub fn get_mut() -> Result<Lock<'static, Self>, DbError> {
        let key = Self::ID;
        database()?.get_mut(&key)
    }

    pub fn put(&self) -> Result<(), DbError> {
        let key = Self::ID;
        database()?.put(&key, self)
    }
}
