use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Ethereum,
    Local,
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
pub struct SequencingInfos(BTreeMap<(Platform, ServiceProvider), SequencingInfoPayload>);

impl SequencingInfos {
    pub fn sequencing_infos(
        &self,
    ) -> &BTreeMap<(Platform, ServiceProvider), SequencingInfoPayload> {
        &self.0
    }

    pub fn insert(&mut self, key: (Platform, ServiceProvider), value: SequencingInfoPayload) {
        self.0.insert(key, value);
    }
}
