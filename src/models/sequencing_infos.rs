use std::collections::BTreeMap;

use crate::{models::prelude::*, sequencer_types::prelude::SequencingInfoPayload};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SequencingInfos(BTreeMap<String, SequencingInfoPayload>);

impl SequencingInfos {
    pub fn new(sequencing_infos: BTreeMap<String, SequencingInfoPayload>) -> Self {
        Self(sequencing_infos)
    }

    pub fn sequencing_infos(&self) -> &BTreeMap<String, SequencingInfoPayload> {
        &self.0
    }

    pub fn insert(&mut self, key: String, value: SequencingInfoPayload) {
        self.0.insert(key, value);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequencingInfosModel;

impl SequencingInfosModel {
    pub const ID: &'static str = stringify!(SequencingInfosModel);

    pub fn get() -> Result<SequencingInfos, DbError> {
        let key = Self::ID;
        database()?.get(&key)
    }

    pub fn get_or_default() -> Result<SequencingInfos, DbError> {
        let key = Self::ID;
        database()?.get_or_default(&key)
    }

    pub fn get_mut_or_default() -> Result<Lock<'static, SequencingInfos>, DbError> {
        let key = Self::ID;
        match database()?.get_mut(&key) {
            Ok(sequencing_infos) => Ok(sequencing_infos),
            Err(error) => {
                if error.is_none_type() {
                    let sequencing_infos = SequencingInfos::default();
                    database()?.put(&key, &sequencing_infos)?;

                    database()?.get_mut(&key)
                } else {
                    Err(error)
                }
            }
        }
    }

    pub fn get_mut() -> Result<Lock<'static, SequencingInfos>, DbError> {
        let key = Self::ID;
        database()?.get_mut(&key)
    }

    pub fn put(sequencing_infos: &SequencingInfos) -> Result<(), DbError> {
        let key = Self::ID;
        database()?.put(&key, sequencing_infos)
    }
}
