use clap::{Parser, Subcommand};
use radius_sdk::{
    json_rpc::server::RpcServer,
    kvstore::{CachedKvStore, KvStore},
};
use seeder::{
    client::liveness,
    error::Error,
    rpc::{external, internal},
    state::AppState,
    types::*,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("{:?}", panic_info);
    }));

    let mut cli = Cli::init();

    match cli.command {
        Commands::Init { ref config_path } => ConfigPath::init(config_path)?,
        Commands::Start {
            ref mut config_option,
        } => {
            let config = Config::load(config_option)?;

            // Initialize the database.
            KvStore::open(config.database_path())?.init();

            let app_state: AppState = AppState::new(
                config.clone(),
                CachedKvStore::default(),
                CachedKvStore::default(),
            );

            initialize_clients(&app_state).await?;
            tracing::info!("Successfully initialized app state.");

            // Initialize the internal rpc server.
            initialize_internal_rpc_server(&app_state).await?;

            tracing::info!("Starting the seeder server..");
            let server_handle = initialize_external_rpc_server(&app_state).await?;
            server_handle.await.unwrap();
        }
    }

    Ok(())
}

async fn initialize_clients(app_state: &AppState) -> Result<(), Error> {
    let sequencing_info_list = SequencingInfoList::get_mut_or(SequencingInfoList::default)?;

    for (platform, service_provider) in sequencing_info_list.iter() {
        let sequencing_info_payload = SequencingInfoPayload::get(*platform, *service_provider)?;
        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(liveness_info) => {
                liveness::radius::LivenessClient::initialize(
                    app_state.clone(),
                    *platform,
                    *service_provider,
                    liveness_info,
                );
            }
            SequencingInfoPayload::Local(_payload) => {
                todo!("Implement 'LivenessClient' for local sequencing.");
            }
        }
    }

    Ok(())
}

async fn initialize_internal_rpc_server(context: &AppState) -> Result<(), Error> {
    let internal_rpc_url = context.config().internal_rpc_url.to_string();

    // Initialize the seeder internal RPC server.
    let internal_rpc_server = RpcServer::new(context.clone())
        .register_rpc_method::<internal::debug::AddRollup>()?
        .register_rpc_method::<internal::AddSequencingInfo>()?
        .register_rpc_method::<internal::GetSequencingInfo>()?
        .register_rpc_method::<internal::GetSequencingInfos>()?
        .init(internal_rpc_url.clone())
        .await?;

    tracing::info!(
        "Successfully started the seeder internal RPC server: {}",
        internal_rpc_url
    );

    tokio::spawn(async move {
        internal_rpc_server.stopped().await;
    });

    Ok(())
}

async fn initialize_external_rpc_server(context: &AppState) -> Result<JoinHandle<()>, Error> {
    let external_rpc_url = anywhere(&context.config().external_port()?);

    // Initialize the seeder internal RPC server.
    let internal_rpc_server = RpcServer::new(context.clone())
        .register_rpc_method::<external::DeregisterTxOrderer>()?
        .register_rpc_method::<external::GetExecutorRpcUrlList>()?
        .register_rpc_method::<external::GetTxOrdererRpcUrl>()?
        .register_rpc_method::<external::GetTxOrdererRpcUrlList>()?
        .register_rpc_method::<external::RegisterTxOrderer>()?
        .init(external_rpc_url.clone())
        .await?;

    tracing::info!(
        "Successfully started the seeder external RPC server: {}",
        external_rpc_url
    );

    let server_handle = tokio::spawn(async move {
        internal_rpc_server.stopped().await;
    });

    Ok(server_handle)
}

pub fn anywhere(port: &str) -> String {
    format!("0.0.0.0:{}", port)
}
