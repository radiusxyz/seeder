use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
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

impl From<IpAddr> for IpAddress {
    fn from(ip: IpAddr) -> Self {
        Self(ip.to_string())
    }
}

impl From<&str> for IpAddress {
    fn from(ip: &str) -> Self {
        Self(ip.to_string())
    }
}
