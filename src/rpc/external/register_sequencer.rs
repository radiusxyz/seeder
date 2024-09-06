use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    // liveness::types::Address,
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::prelude::{SequencerModel, SequencingInfosModel},
    sequencer_types::prelude::*,
    state::AppState,
    util::health_check,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterSequencerMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSequencer {
    signature: Signature,
    message: RegisterSequencerMessage,
}

impl RegisterSequencer {
    pub const METHOD_NAME: &'static str = "register_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<RegisterSequencer>()?;

        // // verify siganture
        // parameter.signature.verify_signature(
        //     rpc::methods::serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.as_slice(),
        //     parameter.message.chain_type,
        // )?;

        let sequencing_key = sequencing_key(
            parameter.message.platform,
            parameter.message.service_provider,
        );

        let sequencing_info = SequencingInfosModel::get()?;
        let sequencing_info_payload = sequencing_info
            .sequencing_infos()
            .get(&sequencing_key)
            .ok_or(Error::FailedToGetSequencingInfo)?;

        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(_payload) => {
                let publisher = context.get_publisher(&sequencing_key).await?;

                let block_number = publisher.get_block_number().await?;
                let sequencer_list = publisher
                    .get_sequencer_list(&parameter.message.cluster_id, block_number)
                    .await?;

                // check if the sequencer is registered in the contract
                sequencer_list
                    .iter()
                    .find(|&address| address.as_slice() == parameter.message.address)
                    .ok_or(Error::UnRegisteredFromContract)?;
            }
            _ => {}
        }

        // health check
        health_check(parameter.message.rpc_url.as_str()).await?;

        match SequencerModel::get(&parameter.message.address) {
            Ok(sequencer) => {
                tracing::warn!("Already registered sequencer: {:?}", sequencer);

                let sequencer =
                    SequencerModel::new(parameter.message.address, Some(parameter.message.rpc_url));

                sequencer.put()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    let sequencer = SequencerModel::new(
                        parameter.message.address,
                        Some(parameter.message.rpc_url),
                    );

                    sequencer.put()?;
                    tracing::info!("Added sequencer: {:?}", sequencer);
                } else {
                    tracing::error!("Failed to add sequencer: {:?}", err);
                    return Err(err.into());
                }
            }
        };

        Ok(())
    }
}
