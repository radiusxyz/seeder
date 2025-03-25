use std::{
    collections::btree_set::{BTreeSet, Iter},
    str::FromStr,
};

use crate::{
    error::Error,
    types::prelude::{ChainType, Deserialize, Model, Serialize},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Ethereum,
    Local,
}

impl From<Platform> for ChainType {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Ethereum => ChainType::Ethereum,
            Platform::Local => ChainType::Ethereum,
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ethereum" | "Ethereum" => Ok(Self::Ethereum),
            "local" | "Local" => Ok(Self::Local),
            _ => Err(Error::UnsupportedPlatform),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LivenessServiceProvider {
    Radius,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationServiceProvider {
    EigenLayer,
    Symbiotic,
}

impl FromStr for ValidationServiceProvider {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eigen_layer" | "eigenlayer" => Ok(Self::EigenLayer),
            "symbiotic" => Ok(Self::Symbiotic),
            _ => Ok(Self::Symbiotic), // Using default value // TODO: error handling
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Model)]
#[kvstore(key(platform: Platform, liveness_service_provider: LivenessServiceProvider))]
#[serde(untagged)]
pub enum LivenessInfoPayload {
    Ethereum(LivenessRadius),
    Local(LivenessLocal),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LivenessRadius {
    pub liveness_rpc_url: String,
    pub liveness_websocket_url: String,
    pub contract_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LivenessLocal(serde_json::Value);

#[derive(Clone, Debug, Default, Deserialize, Serialize, Model)]
#[kvstore(key())]
pub struct LivenessInfoList(BTreeSet<(Platform, LivenessServiceProvider)>);

impl LivenessInfoList {
    pub fn insert(
        &mut self,
        platform: Platform,
        liveness_service_provider: LivenessServiceProvider,
    ) {
        self.0.insert((platform, liveness_service_provider));
    }

    pub fn remove(
        &mut self,
        platform: Platform,
        liveness_service_provider: LivenessServiceProvider,
    ) {
        self.0.remove(&(platform, liveness_service_provider));
    }

    pub fn iter(&self) -> Iter<'_, (Platform, LivenessServiceProvider)> {
        self.0.iter()
    }
}
