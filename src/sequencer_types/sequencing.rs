use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Ethereum,
    Local,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
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

// Todo: change
pub fn sequencing_key(platform: Platform, service_provider: ServiceProvider) -> String {
    format!("{:?}{:?}", platform, service_provider)
}
