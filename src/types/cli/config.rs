use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    types::cli::{ConfigOption, ConfigPath, CONFIG_FILE_NAME},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    path: PathBuf,
    seeder_rpc_url: String,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, Error> {
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
        let config_string =
            fs::read_to_string(config_file_path).map_err(Error::LoadConfigOption)?;

        // Parse String to TOML String
        let config_file: ConfigOption =
            toml::from_str(&config_string).map_err(Error::ParseTomlString)?;

        // Merge configs from CLI input
        let merged_config_option = config_file.merge(config_option);

        Ok(Config {
            path: config_path,
            seeder_rpc_url: merged_config_option.seeder_rpc_url.unwrap(),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn seeder_rpc_url(&self) -> &String {
        &self.seeder_rpc_url
    }
}