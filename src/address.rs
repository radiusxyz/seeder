use radius_sequencer_sdk::signature::Platform;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "AddressInner")]
pub enum Address {
    String(String),
    Array(Vec<u8>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AddressInner(serde_json::Value);

impl From<String> for Address {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<u8>> for Address {
    fn from(value: Vec<u8>) -> Self {
        Self::Array(value)
    }
}

impl TryFrom<AddressInner> for Address {
    type Error = String;

    fn try_from(value: AddressInner) -> Result<Self, Self::Error> {
        if value.0.is_string() {
            Ok(serde_json::from_value::<String>(value.0)
                .map_err(|error| error.to_string())?
                .into())
        } else if value.0.is_array() {
            Ok(serde_json::from_value::<Vec<u8>>(value.0)
                .map_err(|error| error.to_string())?
                .into())
        } else {
            return Err(String::from("Expected either 'String' or 'Array'."));
        }
    }
}

impl Address {
    pub fn to_sdk_address(
        &self,
        platform: Platform,
    ) -> Result<
        radius_sequencer_sdk::signature::Address,
        radius_sequencer_sdk::signature::SignatureError,
    > {
        match self {
            Self::String(address_string) => {
                radius_sequencer_sdk::signature::Address::from_str(platform, address_string)
            }
            Self::Array(address_array) => {
                radius_sequencer_sdk::signature::Address::from_slice(platform, address_array)
            }
        }
    }
}
