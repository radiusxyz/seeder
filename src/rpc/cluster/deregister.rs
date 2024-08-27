use std::sync::Arc;

use radius_sequencer_sdk::{
    json_rpc::{types::RpcParameter, RpcError},
    liveness::{publisher::Publisher, types::hex},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::prelude::{LivenessClusterModel, SequencerModel, ValidationClusterModel},
    sequencer_types::prelude::*,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DeregisterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterResponse {
    pub success: bool,
}

impl Deregister {
    pub const METHOD_NAME: &'static str = "deregister";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<Publisher>,
    ) -> Result<DeregisterResponse, RpcError> {
        let parameter = parameter.parse::<Deregister>()?;

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        if context
            .is_registered(parameter.message.cluster_id.clone())
            .await?
        {
            tracing::error!(
                "Not deregistered on the Liveness contract. Skipping the deregistration.."
            );

            // Todo: change return error
            return Ok(DeregisterResponse { success: false });
        }

        let address = Address::from(hex::encode(&parameter.message.address));

        let platform = match parameter.message.chain_type {
            ChainType::Ethereum => PlatForm::Ethereum,
            _ => PlatForm::Local,
        };

        // remove liveness cluster
        match parameter.message.sequencing_function_type {
            SequencingFunctionType::Liveness => {
                let mut liveness_cluster_model = LivenessClusterModel::get_mut(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?;

                liveness_cluster_model.remove_sequencer(&address);
                liveness_cluster_model.update()?;
            }

            SequencingFunctionType::Validation => {
                let mut validation_cluster_model = ValidationClusterModel::get_mut(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?;

                validation_cluster_model.remove_validator(&address);
                validation_cluster_model.update()?;
            }
        }

        // remove operator model
        match SequencerModel::get_mut(Address::from(hex::encode(&parameter.message.address))) {
            Ok(sequencer) => {
                sequencer.delete()?;
                sequencer.update()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    tracing::warn!("Already deregistered sequencer");
                    return Ok(DeregisterResponse { success: false });
                } else {
                    return Err(err.into());
                }
            }
        }

        Ok(DeregisterResponse { success: true })
    }
}
