use std::{collections::BTreeMap, sync::Arc};

use radius_sequencer_sdk::{
    json_rpc::RpcServer, kvstore::KvStore as Database, liveness::publisher::Publisher,
};
use seeder::{
    cli::{Cli, Commands, Config, ConfigPath, DATABASE_DIR_NAME},
    error::Error,
    models::prelude::SequencingInfosModel,
    rpc::methods::*,
    sequencer_types::prelude::SequencingInfoPayload,
    state::AppState,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let mut cli = Cli::init();

    match cli.command {
        Commands::Init { ref config_path } => ConfigPath::init(config_path)?,
        Commands::Start {
            ref mut config_option,
        } => {
            let config = Config::load(config_option)?;

            let seeder_rpc_url = config.seeder_rpc_url();

            // Initialize a local database.
            Database::new(config.path().join(DATABASE_DIR_NAME))?.init();

            let app_state = initialize_app_state().await?;

            tracing::info!("Successfully initialized app state.");

            let rpc_server_handle = RpcServer::new(app_state)
                .register_rpc_method(AddRollup::METHOD_NAME, AddRollup::handler)?
                .register_rpc_method(
                    DeregisterSequencer::METHOD_NAME,
                    DeregisterSequencer::handler,
                )?
                .register_rpc_method(GetClusterInfo::METHOD_NAME, GetClusterInfo::handler)?
                .register_rpc_method(GetSequencerRpcUrl::METHOD_NAME, GetSequencerRpcUrl::handler)?
                .register_rpc_method(
                    GetSequencerRpcUrlList::METHOD_NAME,
                    GetSequencerRpcUrlList::handler,
                )?
                .register_rpc_method(
                    GetSequencerRpcUrlListAtBlockHeight::METHOD_NAME,
                    GetSequencerRpcUrlListAtBlockHeight::handler,
                )?
                .register_rpc_method(RegisterSequencer::METHOD_NAME, RegisterSequencer::handler)?
                .register_rpc_method(AddSequencingInfo::METHOD_NAME, AddSequencingInfo::handler)?
                .register_rpc_method(GetSequencingInfo::METHOD_NAME, GetSequencingInfo::handler)?
                .register_rpc_method(GetSequencingInfos::METHOD_NAME, GetSequencingInfos::handler)?
                .init(seeder_rpc_url)
                .await?;

            info!("Seeder server starting at {}", seeder_rpc_url);
            rpc_server_handle.stopped().await;
        }
    }

    Ok(())
}

async fn initialize_app_state() -> Result<AppState, Error> {
    // init app state
    let app_state = AppState::new(BTreeMap::new());

    // get or init sequencing info
    let sequencing_infos = match SequencingInfosModel::get() {
        Ok(sequencing_infos) => sequencing_infos,
        Err(err) => {
            if err.is_none_type() {
                // if is none, init sequencing_info
                let sequencing_info = SequencingInfosModel::default();
                sequencing_info.put()?;
                sequencing_info
            } else {
                return Err(err.into());
            }
        }
    };

    for (key, sequencing_info_payload) in sequencing_infos.sequencing_infos() {
        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(payload) => {
                // init publisher
                if app_state.get_publisher(key).await.is_ok() {
                    continue;
                }

                let publisher = Publisher::new(
                    payload.rpc_url.clone(),
                    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                    payload.contract_address.clone(),
                )
                .map_err(Error::InitializePublisher)?;

                app_state
                    .add_publisher(key.to_string(), Arc::new(publisher))
                    .await;
            }
            _ => {}
        }
    }

    Ok(app_state)
}
