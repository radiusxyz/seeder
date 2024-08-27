pub use serde::{Deserialize, Serialize};

use crate::{
    models::prelude::*,
    sequencer_types::prelude::{Address, IpAddress},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerModel {
    pub address: Address,
    pub rpc_url: Option<IpAddress>,
}

impl SequencerModel {
    pub fn new(address: Address, rpc_url: Option<IpAddress>) -> Self {
        Self { address, rpc_url }
    }
}
impl SequencerModel {
    pub const ID: &'static str = stringify!(OperatorModel);

    pub fn get(address: &Address) -> Result<Self, DbError> {
        let key = (Self::ID, address);
        database()?.get(&key)
    }

    pub fn get_mut(address: &Address) -> Result<Lock<'static, Self>, DbError> {
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