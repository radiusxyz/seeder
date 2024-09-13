use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// TODO: merge with radius_sequencer_sdk::signature::Platform
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Ethereum,
    Local,
}

impl From<Platform> for radius_sequencer_sdk::signature::Platform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Ethereum => Self::Ethereum,
            Platform::Local => Self::Ethereum, // TODO:
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ServiceProvider {
    Radius,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(untagged)] - Deseiralize error: DeserializeAnyNotSupported
pub enum SequencingInfoPayload {
    Ethereum(LivenessEthereum),
    Local(LivenessLocal),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LivenessEthereum {
    pub rpc_url: String,
    pub websocket_url: String,
    pub contract_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LivenessLocal {
    pub rpc_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SequencingInfos(HashMap<(Platform, ServiceProvider), SequencingInfoPayload>);

impl SequencingInfos {
    pub fn sequencing_infos(&self) -> &HashMap<(Platform, ServiceProvider), SequencingInfoPayload> {
        &self.0
    }

    pub fn insert(&mut self, key: (Platform, ServiceProvider), value: SequencingInfoPayload) {
        self.0.insert(key, value);
    }
}
