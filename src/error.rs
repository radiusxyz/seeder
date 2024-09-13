pub use radius_sequencer_sdk::kvstore::KvStoreError;

pub enum Error {
    Boxed(Box<dyn std::error::Error>),
    OpenConfig(std::io::Error),
    ParseConfig(toml::de::Error),
    Database(KvStoreError),
    JsonRPC(radius_sequencer_sdk::json_rpc::Error),
    SignatureError(radius_sequencer_sdk::signature::SignatureError),
    SignatureMismatch,

    Deserialize(serde_json::Error),

    RemoveConfigDirectory,
    CreateConfigDirectory,
    CreateConfigFile,
    CreatePrivateKeyFile,
    LoadConfigOption(std::io::Error),
    ParseTomlString(toml::de::Error),

    ParseContractAddress,

    UnRegisteredFromContract,
    NotDeregisteredFromContract,

    FailedToGetSequencingInfo,
    FailedToGetPublisher,
    FailedToGetSequencer,
    PublisherAlreadyExists,
    ClusterNotRegistered,
    SequencerNotRegistered,

    ExistSequencingInfo,

    AlreadyRegisteredCluster,
    AlreadyRegisteredSequencer,
    AlreadyRegisteredRollup,

    InvalidURL(reqwest::Error),
    PortConnection(reqwest::Error),

    InitializePublisher(radius_sequencer_sdk::liveness_radius::publisher::PublisherError),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boxed(error) => write!(f, "{}", error),
            Self::OpenConfig(error) => write!(f, "{}", error),
            Self::ParseConfig(error) => write!(f, "{}", error),
            Self::Database(error) => write!(f, "{}", error),
            Self::SignatureError(error) => write!(f, "{}", error),
            Self::JsonRPC(error) => write!(f, "{}", error),
            Self::SignatureMismatch => write!(f, "Sender is not the signer."),
            Self::InvalidURL(error) => {
                write!(f, "Health-check failed. The URL is invalid: {}", error,)
            }
            Self::Deserialize(error) => write!(f, "Failed to deserialize: {}", error),
            Self::PortConnection(error) => {
                write!(f, "Health-check failed. Make sure the sequencer is running and port-forwarded: {}", error)
            }
            Self::RemoveConfigDirectory => {
                write!(f, "Failed to remove the previous configuration directory")
            }
            Self::ClusterNotRegistered => {
                write!(f, "Cluster not found")
            }
            Self::FailedToGetSequencer => {
                write!(f, "Failed to get rpc url")
            }
            Self::AlreadyRegisteredCluster => {
                write!(f, "Cluster already registered")
            }
            Self::AlreadyRegisteredSequencer => {
                write!(f, "Sequencer already registered")
            }
            Self::AlreadyRegisteredRollup => {
                write!(f, "Rollup already registered")
            }
            Self::SequencerNotRegistered => {
                write!(f, "Sequencer not found in the cluster")
            }
            Self::CreateConfigDirectory => {
                write!(f, "Failed to create a new configuration directory")
            }
            Self::CreateConfigFile => {
                write!(f, "Failed to create a new config file")
            }
            Self::CreatePrivateKeyFile => {
                write!(f, "Failed to create a private key file")
            }
            Self::LoadConfigOption(error) => {
                write!(f, "Failed to load a config file: {}", error)
            }
            Self::ParseTomlString(error) => {
                write!(f, "Failed to parse String to TOML String: {}", error)
            }
            Self::ExistSequencingInfo => {
                write!(f, "Sequencing info already exists")
            }
            Self::FailedToGetSequencingInfo => {
                write!(f, "Failed to get sequencing info")
            }
            Self::FailedToGetPublisher => {
                write!(f, "Failed to get publisher")
            }
            Self::PublisherAlreadyExists => {
                write!(f, "Publisher already exists")
            }
            Self::ParseContractAddress => {
                write!(f, "Failed to parse contract address")
            }
            Self::UnRegisteredFromContract => {
                write!(f, "Sequencer is not registered from the contract")
            }
            Self::NotDeregisteredFromContract => {
                write!(f, "Sequencer is not deregistered from the contract")
            }
            Self::InitializePublisher(error) => {
                write!(f, "Failed to initialize publisher: {:?}", error)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<KvStoreError> for Error {
    fn from(value: KvStoreError) -> Self {
        Self::Database(value)
    }
}

impl From<radius_sequencer_sdk::json_rpc::Error> for Error {
    fn from(value: radius_sequencer_sdk::json_rpc::Error) -> Self {
        Self::JsonRPC(value)
    }
}

impl From<radius_sequencer_sdk::signature::SignatureError> for Error {
    fn from(value: radius_sequencer_sdk::signature::SignatureError) -> Self {
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
