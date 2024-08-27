use radius_sequencer_sdk::liveness::types::Address as AlloyAddress;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IpAddress(String);

impl std::fmt::Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for IpAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
pub struct Address(String);

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Address {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_owned())
    }
}

impl From<String> for Address {
    fn from(address: String) -> Self {
        Self(address)
    }
}

impl From<&str> for Address {
    fn from(address: &str) -> Self {
        Self(address.to_owned())
    }
}

// TODO:
impl PartialEq<AlloyAddress> for Address {
    fn eq(&self, other: &AlloyAddress) -> bool {
        self.0.as_bytes() == other
    }
}
