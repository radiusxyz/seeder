use radius_sequencer_sdk::signature::{Address, Platform};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "SequencerAddressInner")]
pub enum SequencerAddress {
    String(String),
    Array(Vec<u8>),
}

impl Default for SequencerAddress {
    fn default() -> Self {
        Self::Array([0u8; 20].to_vec())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SequencerAddressInner(serde_json::Value);

impl From<String> for SequencerAddress {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<u8>> for SequencerAddress {
    fn from(value: Vec<u8>) -> Self {
        Self::Array(value)
    }
}

impl TryFrom<SequencerAddressInner> for SequencerAddress {
    type Error = String;

    fn try_from(value: SequencerAddressInner) -> Result<Self, Self::Error> {
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

impl SequencerAddress {
    pub fn to_sdk_address(&self, platform: Platform) -> Address {
        match self {
            Self::String(address_string) => Address::from_str(platform, address_string).unwrap(),
            Self::Array(address_array) => Address::from(address_array.clone()),
        }
    }
}
