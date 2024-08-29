use radius_sequencer_sdk::{
    json_rpc::RpcServer, kvstore::KvStore as Database, liveness::publisher::Publisher,
};
use seeder::{
    cli::{Cli, Commands, Config, ConfigPath, DATABASE_DIR_NAME},
    error::Error,
    rpc::methods::*,
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
            Database::new(config.path().join(DATABASE_DIR_NAME))
                .map_err(Error::Database)?
                .init();

            // TODO
            let publisher = Publisher::new(
                "http://127.0.0.1:8545",
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                "0x5FbDB2315678afecb367f032d93F642f64180aa3",
            )
            .map_err(Error::InitializePublisher)?;

            tracing::info!("Successfully initialized the publisher.");

            let rpc_server_handle = RpcServer::new(publisher)
                .register_rpc_method(Deregister::METHOD_NAME, Deregister::handler)?
                .register_rpc_method(GetRpcUrl::METHOD_NAME, GetRpcUrl::handler)?
                .register_rpc_method(GetRpcUrlList::METHOD_NAME, GetRpcUrlList::handler)?
                .register_rpc_method(
                    GetRpcUrlListForRollup::METHOD_NAME,
                    GetRpcUrlListForRollup::handler,
                )?
                .register_rpc_method(Register::METHOD_NAME, Register::handler)?
                .init(seeder_rpc_url)
                .await?;

            info!("Seeder server starting at {}", seeder_rpc_url);
            rpc_server_handle.stopped().await;
        }
    }

    Ok(())
}
