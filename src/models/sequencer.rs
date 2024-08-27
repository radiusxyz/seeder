pub use serde::{Deserialize, Serialize};

use crate::{models::prelude::*, sequencer_types::prelude::IpAddress};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerModel {
    pub address: Vec<u8>,
    pub rpc_url: Option<IpAddress>,
}

impl SequencerModel {
    pub fn new(address: Vec<u8>, rpc_url: Option<IpAddress>) -> Self {
        Self { address, rpc_url }
    }
}
impl SequencerModel {
    pub const ID: &'static str = stringify!(OperatorModel);

    pub fn get(address: &Vec<u8>) -> Result<Self, DbError> {
        let key = (Self::ID, address);
        database()?.get(&key)
    }

    pub fn get_mut(address: &Vec<u8>) -> Result<Lock<'static, Self>, DbError> {
        let key = (Self::ID, address);
        database()?.get_mut(&key)
    }

    pub fn put(&self) -> Result<(), DbError> {
        let key = (Self::ID, self.address.clone());
        database()?.put(&key, self)
    }

    pub fn delete(&self) -> Result<(), DbError> {
        let key = (Self::ID, self.address.clone());
        database()?.delete(&key)
    }
}
