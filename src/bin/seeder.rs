use std::{collections::HashMap, sync::Arc};

use debug::AddRollup;
use radius_sdk::{
    json_rpc::server::RpcServer, kvstore::KvStore, liveness_radius::publisher::Publisher,
};
use seeder::{error::Error, rpc::*, state::AppState, types::*};
use tokio::task::JoinHandle;

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

            // Initialize a local database.
            KvStore::new(config.path().join(DATABASE_DIR_NAME))?.init();

            let app_state = initialize_app_state(DEFAULT_SIGNING_KEY).await?;
            tracing::info!("Successfully initialized app state.");

            initialize_internal_rpc_server(&app_state, config.seeder_internal_rpc_url()).await?;

            let server_handle =
                initialize_external_rpc_server(&app_state, config.seeder_external_rpc_url())
                    .await?;

            tracing::info!("Seeder server started");

            server_handle.await.unwrap();
        }
    }

    Ok(())
}

async fn initialize_app_state(signing_key: &str) -> Result<AppState, Error> {
    // init app state
    let app_state = AppState::new(HashMap::new());

    let sequencing_info_list = SequencingInfoList::get_mut_or(SequencingInfoList::default)?;

    for (platform, service_provider) in sequencing_info_list.iter() {
        let sequencing_info_key = (*platform, *service_provider);
        let sequencing_info_payload = SequencingInfoPayload::get(*platform, *service_provider)?;
        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(payload) => {
                // init publisher
                if app_state.get_publisher(&sequencing_info_key).await.is_ok() {
                    continue;
                }

                // TODO: remove hard-coded value
                let publisher = Publisher::new(
                    payload.liveness_rpc_url.clone(),
                    signing_key,
                    payload.contract_address.clone(),
                )
                .map_err(Error::InitializePublisher)?;

                app_state
                    .add_publisher(sequencing_info_key, Arc::new(publisher))
                    .await;
            }
            _ => {}
        }
    }

    Ok(app_state)
}

async fn initialize_internal_rpc_server(
    context: &AppState,
    seeder_internal_rpc_url: &String,
) -> Result<(), Error> {
    // Initialize the seeder internal RPC server.
    let internal_rpc_server = RpcServer::new(context.clone())
        .register_rpc_method(AddRollup::METHOD_NAME, AddRollup::handler)?
        .register_rpc_method(AddSequencingInfo::METHOD_NAME, AddSequencingInfo::handler)?
        .register_rpc_method(GetSequencingInfo::METHOD_NAME, GetSequencingInfo::handler)?
        .register_rpc_method(GetSequencingInfos::METHOD_NAME, GetSequencingInfos::handler)?
        .init(seeder_internal_rpc_url)
        .await?;

    tracing::info!(
        "Successfully started the seeder internal RPC server: {}",
        seeder_internal_rpc_url
    );

    tokio::spawn(async move {
        internal_rpc_server.stopped().await;
    });

    Ok(())
}

async fn initialize_external_rpc_server(
    context: &AppState,
    seeder_external_rpc_url: &String,
) -> Result<JoinHandle<()>, Error> {
    // Initialize the seeder internal RPC server.
    let internal_rpc_server = RpcServer::new(context.clone())
        .register_rpc_method(
            DeregisterSequencer::METHOD_NAME,
            DeregisterSequencer::handler,
        )?
        .register_rpc_method(
            GetExecutorRpcUrlList::METHOD_NAME,
            GetExecutorRpcUrlList::handler,
        )?
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
        .init(seeder_external_rpc_url)
        .await?;

    tracing::info!(
        "Successfully started the seeder external RPC server: {}",
        seeder_external_rpc_url
    );

    let server_handle = tokio::spawn(async move {
        internal_rpc_server.stopped().await;
    });

    Ok(server_handle)
}
