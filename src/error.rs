#[derive(Debug)]
pub enum Error {
    Boxed(Box<dyn std::error::Error>),
    Config(crate::types::ConfigError),
    Database(radius_sdk::kvstore::KvStoreError),
    JsonRPC(radius_sdk::json_rpc::server::RpcServerError),
    SignatureError(radius_sdk::signature::SignatureError),

    NotRegisteredInContract,
    NotDeregisteredFromContract,
    FailedToGetPublisher,

    UnsupportedPlatform,
    UnsupportedValidationServiceProvider,
    UnsupportedRollupType,

    PublisherAlreadyExists,

    InvalidURL(reqwest::Error),
    PortConnection(reqwest::Error),

    InitializePublisher(radius_sdk::liveness_radius::publisher::PublisherError),
}

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
        Self::JsonRPC(value)
    }
}

impl From<radius_sdk::signature::SignatureError> for Error {
    fn from(value: radius_sdk::signature::SignatureError) -> Self {
        Self::SignatureError(value)
    }
}

impl Error {
    pub fn boxed<E>(error: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        Self::Boxed(Box::new(error))
    }
}
