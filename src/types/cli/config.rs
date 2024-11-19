use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::types::cli::{ConfigOption, ConfigPath, CONFIG_FILE_NAME};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    path: PathBuf,
    external_rpc_url: String,
    internal_rpc_url: String,
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

        Ok(Config {
            path: config_path,
            external_rpc_url: merged_config_option
                .seeder_external_rpc_url
                .ok_or(ConfigError::EmptyExternalRpcUrl)?,
            internal_rpc_url: merged_config_option
                .seeder_internal_rpc_url
                .ok_or(ConfigError::EmptyInternalRpcUrl)?,
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn external_rpc_url(&self) -> &String {
        &self.external_rpc_url
    }

    pub fn internal_rpc_url(&self) -> &String {
        &self.internal_rpc_url
    }

    pub fn external_port(&self) -> Result<String, ConfigError> {
        Ok(self
            .external_rpc_url()
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
    RemoveConfigDirectory(std::io::Error),
    CreateConfigDirectory(std::io::Error),
    CreateConfigFile(std::io::Error),

    InvalidExternalPort,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ConfigError {}
