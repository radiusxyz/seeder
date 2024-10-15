#[derive(Debug)]
pub enum Error {
    Boxed(Box<dyn std::error::Error>),
    OpenConfig(std::io::Error),
    ParseConfig(toml::de::Error),
    Database(radius_sdk::kvstore::KvStoreError),
    JsonRPC(radius_sdk::json_rpc::Error),
    SignatureError(radius_sdk::signature::SignatureError),
    SignatureMismatch,

    Deserialize(serde_json::Error),

    RemoveConfigDirectory,
    CreateConfigDirectory,
    CreateConfigFile,
    CreatePrivateKeyFile,
    LoadConfigOption(std::io::Error),
    ParseTomlString(toml::de::Error),
    ParseContractAddress,

    NotRegisteredInContract,
    NotDeregisteredFromContract,

    FailedToGetSequencingInfo,
    FailedToGetPublisher,
    FailedToGetSequencer,

    NotSupportedPlatform,
    NotSupportedValidationServiceProvider,
    NotSupportedRollupType,

    PublisherAlreadyExists,
    SequencingInfoAlreadyExists,

    UnsupportedChainType(crate::types::Platform),

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

impl From<radius_sdk::kvstore::KvStoreError> for Error {
    fn from(value: radius_sdk::kvstore::KvStoreError) -> Self {
        Self::Database(value)
    }
}

impl From<radius_sdk::json_rpc::Error> for Error {
    fn from(value: radius_sdk::json_rpc::Error) -> Self {
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
