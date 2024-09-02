use crate::{
    models::prelude::*,
    sequencer_types::prelude::{SequencingInfoKey, SequencingInfoKeyList},
};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SequencingInfoKeyListModel {
    sequencing_info_key_list: SequencingInfoKeyList,
}

impl SequencingInfoKeyListModel {
    pub fn new(sequencing_info_key_list: SequencingInfoKeyList) -> Self {
        Self {
            sequencing_info_key_list,
        }
    }

    pub fn sequencing_info_key_list(self) -> SequencingInfoKeyList {
        self.sequencing_info_key_list
    }

    pub fn add_sequencing_info_key(&mut self, sequencing_info_key: SequencingInfoKey) {
        if !self.is_exist_sequencing_info_key(&sequencing_info_key) {
            self.sequencing_info_key_list
                .as_mut()
                .push(sequencing_info_key);
        }
    }

    pub fn is_exist_sequencing_info_key(&self, cluster_id: &SequencingInfoKey) -> bool {
        self.sequencing_info_key_list.as_ref().contains(cluster_id)
    }
}

impl SequencingInfoKeyListModel {
    pub const ID: &'static str = stringify!(SequencingInfoKeyListModel);

    pub fn get() -> Result<Self, DbError> {
        let key = Self::ID;
        database()?.get(&key)
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
