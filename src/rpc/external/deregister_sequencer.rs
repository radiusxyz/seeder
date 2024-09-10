use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, state::AppState, types::prelude::*};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeregisterSequencerMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterSequencer {
    signature: Signature,
    message: DeregisterSequencerMessage,
}

impl DeregisterSequencer {
    pub const METHOD_NAME: &'static str = "deregister_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<DeregisterSequencer>()?;

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

                // check if the sequencer is deregistered from the contract
                sequencer_list
                    .iter()
                    .find(|&&address| address.as_slice() == parameter.message.address)
                    .map_or(Ok(()), |_| Err(Error::NotDeregisteredFromContract))?;
            }
            _ => {}
        }

        // remove sequencer model
        match SequencerNodeInfoModel::get_mut(&parameter.message.address) {
            Ok(sequencer_node_info) => {
                SequencerNodeInfoModel::delete(&sequencer_node_info)?;
            }
            Err(err) => {
                if err.is_none_type() {
                    tracing::warn!("Already deregistered sequencer");
                } else {
                    return Err(err.into());
                }
            }
        }

        Ok(())
    }
}
