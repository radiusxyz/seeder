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

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(address_string) => {
                let lowercased = address_string.to_lowercase();
                write!(f, "{}", lowercased)
            }
            Self::Array(address_array) => fmt_hex_string(f, address_array),
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

pub fn fmt_hex_string(f: &mut std::fmt::Formatter, data: &[u8]) -> std::fmt::Result {
    f.write_str("0x")?;
    data.iter()
        .try_for_each(|byte| f.write_fmt(format_args!("{:02x}", byte)))
}
