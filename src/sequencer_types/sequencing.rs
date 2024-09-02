use radius_sequencer_sdk::signature::ChainType;
use serde::{Deserialize, Serialize};

use super::prelude::IpAddress;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Local,
    Ethereum,
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Platform::Local),
            "ethereum" => Ok(Platform::Ethereum),
            _ => Err(format!("unknown platform: {}", s)),
        }
    }
}

impl From<ChainType> for Platform {
    fn from(chain_type: ChainType) -> Self {
        match chain_type {
            ChainType::Ethereum => Platform::Ethereum,
            ChainType::Bitcoin => Platform::Local,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Local => write!(f, "local"),
            Platform::Ethereum => write!(f, "ethereum"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SequencingFunctionType {
    Liveness,
    Validation,
}

impl std::fmt::Display for SequencingFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SequencingFunctionType::Liveness => write!(f, "liveness"),
            SequencingFunctionType::Validation => write!(f, "validation"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ServiceProvider {
    Local,
    Radius,
}

impl std::fmt::Display for ServiceProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceProvider::Local => write!(f, "local"),
            ServiceProvider::Radius => write!(f, "radius"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequencingInfo {
    sequencing_info_key: SequencingInfoKey,
    sequencing_info_payload: SequencingInfoPayload,
}

impl SequencingInfo {
    pub fn new(
        sequencing_info_key: SequencingInfoKey,
        sequencing_info_payload: SequencingInfoPayload,
    ) -> Self {
        Self {
            sequencing_info_key,
            sequencing_info_payload,
        }
    }

    pub fn sequencing_info_key(&self) -> SequencingInfoKey {
        self.sequencing_info_key
    }

    pub fn sequencing_info_payload(&self) -> &SequencingInfoPayload {
        &self.sequencing_info_payload
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SequencingInfoKey(Platform, SequencingFunctionType, ServiceProvider);

impl From<SequencingCondition> for SequencingInfoKey {
    fn from(condition: SequencingCondition) -> Self {
        match condition {
            SequencingCondition::LocalLivenessLocal => SequencingInfoKey(
                Platform::Local,
                SequencingFunctionType::Liveness,
                ServiceProvider::Local,
            ),
            SequencingCondition::LocalLivenessRadius => SequencingInfoKey(
                Platform::Local,
                SequencingFunctionType::Liveness,
                ServiceProvider::Radius,
            ),
            SequencingCondition::LocalValidationLocal => SequencingInfoKey(
                Platform::Local,
                SequencingFunctionType::Validation,
                ServiceProvider::Local,
            ),
            SequencingCondition::LocalValidationRadius => SequencingInfoKey(
                Platform::Local,
                SequencingFunctionType::Validation,
                ServiceProvider::Radius,
            ),
            SequencingCondition::EthereumLivenessLocal => SequencingInfoKey(
                Platform::Ethereum,
                SequencingFunctionType::Liveness,
                ServiceProvider::Local,
            ),
            SequencingCondition::EthereumLivenessRadius => SequencingInfoKey(
                Platform::Ethereum,
                SequencingFunctionType::Liveness,
                ServiceProvider::Radius,
            ),
            SequencingCondition::EthereumValidationLocal => SequencingInfoKey(
                Platform::Ethereum,
                SequencingFunctionType::Validation,
                ServiceProvider::Local,
            ),
            SequencingCondition::EthereumValidationRadius => SequencingInfoKey(
                Platform::Ethereum,
                SequencingFunctionType::Validation,
                ServiceProvider::Radius,
            ),
        }
    }
}

impl std::fmt::Display for SequencingInfoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0, self.1, self.2)
    }
}

impl SequencingInfoKey {
    pub fn new(
        platform: Platform,
        sequencing_function_type: SequencingFunctionType,
        service_provider: ServiceProvider,
    ) -> Self {
        Self(platform, sequencing_function_type, service_provider)
    }

    pub fn platform(self) -> Platform {
        self.0
    }

    pub fn sequencing_function_type(self) -> SequencingFunctionType {
        self.1
    }

    pub fn service_provider(self) -> ServiceProvider {
        self.2
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SequencingInfoKeyList(Vec<SequencingInfoKey>);

impl AsRef<Vec<SequencingInfoKey>> for SequencingInfoKeyList {
    fn as_ref(&self) -> &Vec<SequencingInfoKey> {
        &self.0
    }
}

impl AsMut<Vec<SequencingInfoKey>> for SequencingInfoKeyList {
    fn as_mut(&mut self) -> &mut Vec<SequencingInfoKey> {
        &mut self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct SequencingInfoPayload(
    Option<IpAddress>,
    Option<IpAddress>,
    Option<ContractAddress>,
);

impl std::fmt::Display for SequencingInfoPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "provider_rpc_url: {:?}, provider_websocket_url: {:?}, contract_address: {:?}",
            self.0, self.1, self.2,
        )
    }
}

impl SequencingInfoPayload {
    pub fn new(
        provider_rpc_url: Option<IpAddress>,
        provider_websocket_url: Option<IpAddress>,
        contract_address: Option<ContractAddress>,
    ) -> Self {
        Self(provider_rpc_url, provider_websocket_url, contract_address)
    }

    pub fn provider_rpc_url(&self) -> &Option<IpAddress> {
        &self.0
    }

    pub fn provider_websocket_url(&self) -> &Option<IpAddress> {
        &self.1
    }

    pub fn contract_address(&self) -> &Option<ContractAddress> {
        &self.2
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ContractAddress(String);

impl std::fmt::Display for ContractAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ContractAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ContractAddress {
    fn from(address: &str) -> Self {
        Self(address.to_string())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SequencingCondition {
    LocalLivenessLocal,
    LocalLivenessRadius,
    LocalValidationLocal,
    LocalValidationRadius,
    EthereumLivenessLocal,
    EthereumLivenessRadius,
    EthereumValidationLocal,
    EthereumValidationRadius,
}

impl From<SequencingInfoKey> for SequencingCondition {
    fn from(key: SequencingInfoKey) -> Self {
        match (
            key.platform(),
            key.sequencing_function_type(),
            key.service_provider(),
        ) {
            (Platform::Local, SequencingFunctionType::Liveness, ServiceProvider::Local) => {
                Self::LocalLivenessLocal
            }
            (Platform::Local, SequencingFunctionType::Liveness, ServiceProvider::Radius) => {
                Self::LocalLivenessRadius
            }
            (Platform::Local, SequencingFunctionType::Validation, ServiceProvider::Local) => {
                Self::LocalValidationLocal
            }
            (Platform::Local, SequencingFunctionType::Validation, ServiceProvider::Radius) => {
                Self::LocalValidationRadius
            }
            (Platform::Ethereum, SequencingFunctionType::Liveness, ServiceProvider::Local) => {
                Self::EthereumLivenessLocal
            }
            (Platform::Ethereum, SequencingFunctionType::Liveness, ServiceProvider::Radius) => {
                Self::EthereumLivenessRadius
            }
            (Platform::Ethereum, SequencingFunctionType::Validation, ServiceProvider::Local) => {
                Self::EthereumValidationLocal
            }
            (Platform::Ethereum, SequencingFunctionType::Validation, ServiceProvider::Radius) => {
                Self::EthereumValidationRadius
            }
        }
    }
}
