use std::net::IpAddr;

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

impl From<IpAddr> for IpAddress {
    fn from(ip_url: IpAddr) -> Self {
        Self(ip_url.to_string())
    }
}
