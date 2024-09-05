use std::{collections::HashMap, sync::Arc};

use radius_sequencer_sdk::{
    json_rpc::RpcServer, kvstore::KvStore as Database, liveness::publisher::Publisher,
};
use seeder::{
    cli::{Cli, Commands, Config, ConfigPath, DATABASE_DIR_NAME},
    error::Error,
    models::prelude::{
        ClusterIdListModel, ClusterInfoModel, SequencingInfoKeyListModel, SequencingInfoModel,
    },
    rpc::methods::*,
    sequencer_types::prelude::{SequencingCondition, SequencingInfoPayload},
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

            let app_state = initialize_app_state(&config)?;

            tracing::info!("Successfully initialized app state.");

            let rpc_server_handle = RpcServer::new(app_state)
                .register_rpc_method(AddCluster::METHOD_NAME, AddCluster::handler)?
                .register_rpc_method(
                    DeregisterSequencer::METHOD_NAME,
                    DeregisterSequencer::handler,
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

fn initialize_app_state(config: &Config) -> Result<AppState, Error> {
    // init app state
    let app_state = AppState::new(
        config.clone(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    );

    let sequencing_info_key_list_model = match SequencingInfoKeyListModel::get() {
        Ok(sequencing_info_key_list) => sequencing_info_key_list,
        Err(err) => {
            if err.is_none_type() {
                // if is none, init sequencing_info_key_list
                let sequencing_info_key_list_model = SequencingInfoKeyListModel::default();
                sequencing_info_key_list_model.put()?;
                sequencing_info_key_list_model
            } else {
                return Err(err.into());
            }
        }
    };

    for sequencing_info_key in sequencing_info_key_list_model
        .sequencing_info_key_list()
        .as_ref()
    {
        // get or init sequencing info
        let sequencing_info_model = match SequencingInfoModel::get(*sequencing_info_key) {
            Ok(sequencing_info) => sequencing_info,
            Err(err) => {
                if err.is_none_type() {
                    // init sequencing_info
                    let sequencing_info = SequencingInfoModel::new(
                        *sequencing_info_key,
                        SequencingInfoPayload::default(),
                    );
                    sequencing_info.put(*sequencing_info_key)?;
                    sequencing_info
                } else {
                    return Err(err.into());
                }
            }
        };

        // add sequencing info to app state
        let sequencing_info_payload = sequencing_info_model.sequencing_info_payload();
        app_state.add_sequencing_info(*sequencing_info_key, sequencing_info_payload.clone());

        let cluster_id_list = match ClusterIdListModel::get(
            sequencing_info_key.platform(),
            sequencing_info_key.sequencing_function_type(),
            sequencing_info_key.service_provider(),
        ) {
            Ok(cluster_id_list) => cluster_id_list,
            Err(err) => {
                if err.is_none_type() {
                    // if is none, init cluster_id_list_model
                    let cluster_id_list_model = ClusterIdListModel::default();
                    cluster_id_list_model.put(
                        sequencing_info_key.platform(),
                        sequencing_info_key.sequencing_function_type(),
                        sequencing_info_key.service_provider(),
                    )?;
                    cluster_id_list_model
                } else {
                    return Err(err.into());
                }
            }
        };

        for cluster_id in cluster_id_list.cluster_id_list().as_ref() {
            let cluster_info = match ClusterInfoModel::get(cluster_id) {
                Ok(cluster_info) => cluster_info,
                Err(err) => {
                    if err.is_none_type() {
                        // if is none, init cluster_info_model
                        let cluster_info_model = ClusterInfoModel::new(
                            cluster_id.clone(),
                            *sequencing_info_key,
                            Vec::new(),
                        );
                        cluster_info_model.put()?;
                        cluster_info_model
                    } else {
                        return Err(err.into());
                    }
                }
            };

            app_state.add_cluster_info(cluster_id.clone(), cluster_info.into());
        }

        if matches!(
            SequencingCondition::from(*sequencing_info_key),
            SequencingCondition::EthereumLivenessRadius
        ) {
            // init publisher
            if app_state.get_publisher(*sequencing_info_key).is_ok() {
                continue;
            }

            let publisher = Publisher::new(
                sequencing_info_payload.provider_rpc_url().clone().unwrap(),
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                sequencing_info_payload
                    .provider_websocket_url()
                    .clone()
                    .unwrap(),
            )
            .map_err(Error::InitializePublisher)?;

            app_state.add_publisher(*sequencing_info_key, Arc::new(publisher));
        }
    }

    Ok(app_state)
}
