#[derive(Debug)]
pub enum Error {
    Config(crate::types::ConfigError),
    Database(radius_sdk::kvstore::KvStoreError),
    RpcServer(radius_sdk::json_rpc::server::RpcServerError),
    Signature(radius_sdk::signature::SignatureError),
    LivenessClient(Box<dyn std::error::Error>),
    NotRegisteredInContract,
    NotDeregisteredFromContract,
    UnsupportedPlatform,
    UnsupportedValidationServiceProvider,
    UnsupportedRollupType,
    InvalidURL(reqwest::Error),
    HealthCheck(reqwest::Error),
}

unsafe impl Send for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<crate::types::ConfigError> for Error {
    fn from(value: crate::types::ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<radius_sdk::kvstore::KvStoreError> for Error {
    fn from(value: radius_sdk::kvstore::KvStoreError) -> Self {
        Self::Database(value)
    }
}

impl From<radius_sdk::json_rpc::server::RpcServerError> for Error {
    fn from(value: radius_sdk::json_rpc::server::RpcServerError) -> Self {
        Self::RpcServer(value)
    }
}

impl From<radius_sdk::signature::SignatureError> for Error {
    fn from(value: radius_sdk::signature::SignatureError) -> Self {
        Self::Signature(value)
    }
}
