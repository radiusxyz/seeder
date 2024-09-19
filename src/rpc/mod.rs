mod external;
mod internal;

mod prelude {
    pub use std::sync::Arc;

    pub use radius_sequencer_sdk::{
        json_rpc::{types::*, RpcError},
        signature::{Address, Signature},
    };
    pub use serde::{Deserialize, Serialize};
}

pub mod methods {
    pub use crate::rpc::{external::*, internal::*};

    pub fn serialize_to_bincode<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(value)
    }
}
