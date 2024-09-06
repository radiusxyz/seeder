pub use serde::{Deserialize, Serialize};

use crate::models::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerModel {
    pub sequencer_address: Vec<u8>,
    pub rpc_url: Option<String>,
}

impl SequencerModel {
    pub fn new(sequencer_address: Vec<u8>, rpc_url: Option<String>) -> Self {
        Self {
            sequencer_address,
            rpc_url,
        }
    }
}

impl SequencerModel {
    pub const ID: &'static str = stringify!(SequencerModel);

    pub fn get(address: &[u8]) -> Result<Self, DbError> {
        let key = (Self::ID, address);
        database()?.get(&key)
    }

    pub fn get_mut(address: &[u8]) -> Result<Lock<'static, Self>, DbError> {
        let key = (Self::ID, address);
        database()?.get_mut(&key)
    }

    pub fn put(&self) -> Result<(), DbError> {
        let key = (Self::ID, self.sequencer_address.clone());
        database()?.put(&key, self)
    }

    pub fn delete(&self) -> Result<(), DbError> {
        let key = (Self::ID, self.sequencer_address.clone());
        database()?.delete(&key)
    }
}
