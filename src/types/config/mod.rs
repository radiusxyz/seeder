mod config_option;
mod config_path;

use std::{fs, path::PathBuf};

pub use config_option::*;
pub use config_path::*;
use serde::{Deserialize, Serialize};

pub const DEFAULT_HOME_PATH: &str = ".radius";
pub const DATABASE_DIR_NAME: &str = "database";

pub const CONFIG_FILE_NAME: &str = "Config.toml";
pub const SIGNING_KEY_PATH: &str = "signing_key";
pub const DEFAULT_SIGNING_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub path: PathBuf,

    pub external_rpc_url: String,
    pub internal_rpc_url: String,

    pub signing_key: String,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, ConfigError> {
        let config_path = match config_option.path.as_mut() {
            Some(config_path) => config_path.clone(),
            None => {
                let config_path: PathBuf = ConfigPath::default().as_ref().into();
                config_option.path = Some(config_path.clone());
                config_path
            }
        };

        // Read config file
        let config_file_path = config_path.join(CONFIG_FILE_NAME);
        let config_string = fs::read_to_string(config_file_path).map_err(ConfigError::Load)?;

        // Parse String to TOML String
        let config_file: ConfigOption =
            toml::from_str(&config_string).map_err(ConfigError::Parse)?;

        // Merge configs from CLI input
        let merged_config_option = config_file.merge(config_option);

        // Read signing key
        let signing_key_path = config_path.join(SIGNING_KEY_PATH);
        let signing_key =
            fs::read_to_string(signing_key_path).map_err(|_| ConfigError::EmptySigningKey)?;

        Ok(Self {
            path: config_path,
            external_rpc_url: merged_config_option
                .seeder_external_rpc_url
                .ok_or(ConfigError::EmptyExternalRpcUrl)?,
            internal_rpc_url: merged_config_option
                .seeder_internal_rpc_url
                .ok_or(ConfigError::EmptyInternalRpcUrl)?,
            signing_key,
        })
    }

    pub fn database_path(&self) -> PathBuf {
        self.path.join(DATABASE_DIR_NAME)
    }

    pub fn external_port(&self) -> Result<String, ConfigError> {
        Ok(self
            .external_rpc_url
            .split(':')
            .last()
            .ok_or(ConfigError::InvalidExternalPort)?
            .to_string())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Load(std::io::Error),
    Parse(toml::de::Error),
    EmptyExternalRpcUrl,
    EmptyInternalRpcUrl,
    EmptySigningKey,
    RemoveConfigDirectory(std::io::Error),
    CreateConfigDirectory(std::io::Error),
    CreateConfigFile(std::io::Error),
    CreatePrivateKeyFile(std::io::Error),

    InvalidExternalPort,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ConfigError {}
