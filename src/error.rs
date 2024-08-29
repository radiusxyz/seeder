pub enum Error {
    OpenConfig(std::io::Error),
    ParseConfig(toml::de::Error),
    Database(radius_sequencer_sdk::kvstore::KvStoreError),
    JsonRPC(radius_sequencer_sdk::json_rpc::Error),
    SignatureMismatch,

    RemoveConfigDirectory,
    CreateConfigDirectory,
    CreateConfigFile,
    CreatePrivateKeyFile,
    LoadConfigOption,
    ParseTomlString,

    ParseContractAddress,

    UnregisteredOnContract,
    NotDeregisteredFromContract,

    InvalidURL(reqwest::Error),
    HealthCheck(reqwest::Error),

    InitializePublisher(radius_sequencer_sdk::liveness::publisher::PublisherError),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenConfig(error) => write!(f, "{}", error),
            Self::ParseConfig(error) => write!(f, "{}", error),
            Self::Database(error) => write!(f, "{}", error),
            Self::JsonRPC(error) => write!(f, "{}", error),
            Self::SignatureMismatch => write!(f, "Sender is not the signer."),
            Self::InvalidURL(error) => {
                write!(f, "Health-check failed. The URL is invalid: {}", error,)
            }
            Self::HealthCheck(error) => {
                write!(f, "Health-check failed. Make sure the sequencer is running and port-forwarded: {}", error)
            }
            Self::RemoveConfigDirectory => {
                write!(f, "Failed to remove the previous configuration directory")
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
            Self::LoadConfigOption => {
                write!(f, "Failed to load a config file")
            }
            Self::ParseTomlString => {
                write!(f, "Failed to parse String to TOML String")
            }
            Self::ParseContractAddress => {
                write!(f, "Failed to parse the contract address")
            }
            Self::UnregisteredOnContract => {
                write!(f, "The sequencer is not registered on the contract")
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

impl From<radius_sequencer_sdk::json_rpc::Error> for Error {
    fn from(value: radius_sequencer_sdk::json_rpc::Error) -> Self {
        Self::JsonRPC(value)
    }
}
