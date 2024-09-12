use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "AddressInner")]
pub enum Address {
    String(String),
    Array(Vec<u8>),
}

impl Default for Address {
    fn default() -> Self {
        Self::Array(Vec::default())
    }
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
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::String(address_string) => {
                if let Some(decoded_bytes) = hex_to_bytes(address_string) {
                    decoded_bytes.as_slice().to_vec()
                } else {
                    address_string.as_bytes().to_vec()
                }
            }
            Self::Array(address_array) => address_array.as_slice().to_vec(),
        }
    }
}

fn hex_to_bytes(hex_str: &str) -> Option<Vec<u8>> {
    if hex_str.starts_with("0x") && hex_str.len() == 42 {
        // 20-byte (40 hex chars + "0x")
        (2..hex_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).ok())
            .collect()
    } else if hex_str.len() == 40 {
        (0..hex_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).ok())
            .collect()
    } else {
        None
    }
}
