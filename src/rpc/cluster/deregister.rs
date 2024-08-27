use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::{publisher::Publisher, types::hex},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{error::Error, models::prelude::SequencerModel, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DeregisterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    cluster_id: ClusterId,
}

impl std::fmt::Display for DeregisterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deregister {
    signature: Signature,
    message: DeregisterMessage,
}

impl Deregister {
    pub const METHOD_NAME: &'static str = "deregister";

    pub async fn handler(parameter: RpcParameter, context: Arc<Publisher>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Deregister>()?;

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        let block_number = context.get_block_number().await?;
        let sequencer_list = context
            .get_sequencer_list(&parameter.message.cluster_id, block_number)
            .await?;

        // check if the sequencer is registered
        sequencer_list
            .iter()
            .find(|&address| address.as_slice() == parameter.message.address)
            .ok_or(Error::Deregistered)?;

        // remove operator model
        match SequencerModel::get_mut(&Address::from(hex::encode(&parameter.message.address))) {
            Ok(sequencer) => {
                sequencer.delete()?;
                sequencer.update()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    tracing::warn!("Already deregistered sequencer");
                    return Ok(());
                } else {
                    return Err(err.into());
                }
            }
        }

        Ok(())
    }
}