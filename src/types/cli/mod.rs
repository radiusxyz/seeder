mod config;
mod config_option;
mod config_path;

use clap::{Parser, Subcommand};
pub use config::{Config, ConfigError};
pub use config_option::ConfigOption;
pub use config_path::ConfigPath;
use serde::{Deserialize, Serialize};

pub const DEFAULT_HOME_PATH: &str = ".radius";
pub const DATABASE_DIR_NAME: &str = "database";
pub const CONFIG_FILE_NAME: &str = "Config.toml";
pub const DEFAULT_SIGNING_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug, Deserialize, Parser, Serialize)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}

#[derive(Subcommand, Debug, Deserialize, Serialize)]
pub enum Commands {
    /// Initializes a node
    Init {
        #[clap(flatten)]
        config_path: Box<ConfigPath>,
    },

    /// Starts the node
    Start {
        #[clap(flatten)]
        config_option: Box<ConfigOption>,
    },
}
