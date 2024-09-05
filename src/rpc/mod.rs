mod external;
mod internal;

mod prelude {
    pub use radius_sequencer_sdk::json_rpc::{types::*, RpcError};
}

pub mod methods {
    pub use crate::rpc::{external::*, internal::*};

    pub fn serialize_to_bincode<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(value)
    }
}
