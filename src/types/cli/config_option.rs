use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::ConfigPath;

const DEFAULT_SEEDER_EXTERNAL_RPC_URL: &str = "http://192.168.12.195:6000";
const DEFAULT_SEEDER_INTERNAL_RPC_URL: &str = "http://192.168.12.195:6001";

#[derive(Debug, Deserialize, Parser, Serialize)]
pub struct ConfigOption {
    #[doc = "Set the configuration file path to load from"]
    #[clap(long = "path")]
    pub path: Option<PathBuf>,

    #[doc = "Set the seeder external rpc url"]
    #[clap(long = "seeder-external-rpc-url")]
    pub seeder_external_rpc_url: Option<String>,

    #[doc = "Set the seeder internal rpc url"]
    #[clap(long = "seeder-internal-rpc-url")]
    pub seeder_internal_rpc_url: Option<String>,
}

impl Default for ConfigOption {
    fn default() -> Self {
        Self {
            path: Some(ConfigPath::default().as_ref().into()),
            seeder_external_rpc_url: Some(DEFAULT_SEEDER_EXTERNAL_RPC_URL.into()),
            seeder_internal_rpc_url: Some(DEFAULT_SEEDER_INTERNAL_RPC_URL.into()),
        }
    }
}

impl ConfigOption {
    pub fn get_toml_string(&self) -> String {
        let mut toml_string = String::new();

        set_toml_comment(&mut toml_string, "Set seeder external rpc url");
        set_toml_name_value(
            &mut toml_string,
            "seeder_external_rpc_url",
            &self.seeder_external_rpc_url,
        );

        set_toml_comment(&mut toml_string, "Set seeder internal rpc url");
        set_toml_name_value(
            &mut toml_string,
            "seeder_internal_rpc_url",
            &self.seeder_internal_rpc_url,
        );

        toml_string
    }

    pub fn merge(mut self, other: &ConfigOption) -> Self {
        if other.path.is_some() {
            self.path.clone_from(&other.path)
        }

        if other.seeder_external_rpc_url.is_some() {
            self.seeder_external_rpc_url
                .clone_from(&other.seeder_external_rpc_url)
        }

        if other.seeder_internal_rpc_url.is_some() {
            self.seeder_internal_rpc_url
                .clone_from(&other.seeder_internal_rpc_url)
        }

        self
    }
}

fn set_toml_comment(toml_string: &mut String, comment: &'static str) {
    let comment = format!("# {}\n", comment);
    toml_string.push_str(&comment);
}

fn set_toml_name_value<T>(toml_string: &mut String, name: &'static str, value: &Option<T>)
where
    T: std::fmt::Debug,
{
    let name_value = match value {
        Some(value) => format!("{} = {:?}\n\n", name, value),
        None => format!("# {} = {:?}\n\n", name, value),
    };

    toml_string.push_str(&name_value);
}
