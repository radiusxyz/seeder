use std::{fmt::Formatter, sync::Arc};

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
struct RegisterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
    cluster_id: ClusterId,
    rpc_url: IpAddress,
}

impl std::fmt::Display for RegisterMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Register {
    signature: Signature,
    message: RegisterMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
}

impl Register {
    pub const METHOD_NAME: &'static str = "register";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<Publisher>,
    ) -> Result<RegisterResponse, RpcError> {
        let parameter = parameter.parse::<Register>()?;

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        // TODO: 20byte array -> Bytes array
        if !context
            .is_registered(parameter.message.cluster_id.clone())
            .await?
        {
            tracing::error!("Not registered on the Liveness contract. Skipping the registration..");

            return Ok(RegisterResponse { success: false });
        }

        let platform = match parameter.message.chain_type {
            ChainType::Ethereum => PlatForm::Ethereum,
            _ => PlatForm::Local,
        };

        let address = Address::from(hex::encode(&parameter.message.address));

        match parameter.message.sequencing_function_type {
            SequencingFunctionType::Liveness => {
                let mut liveness_cluster_model = LivenessClusterModel::get_mut(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?;

                liveness_cluster_model.add_seqeuncer(address.clone());
                liveness_cluster_model.update()?;
            }

            SequencingFunctionType::Validation => {
                let mut validation_cluster_model = ValidationClusterModel::get_mut(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?;

                validation_cluster_model.add_seqeuncer(address.clone());
                validation_cluster_model.update()?;
            }
        }

        match SequencerModel::get(Address::from(hex::encode(&parameter.message.address))) {
            // TODO: change(tmp logic when already registered)
            Ok(sequencer) => {
                tracing::warn!("Already registered sequencer: {:?}", sequencer);

                let sequencer = SequencerModel::new(address, parameter.message.rpc_url.into());

                sequencer.put()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    let sequencer = SequencerModel::new(address, parameter.message.rpc_url.into());

                    sequencer.put()?;
                } else {
                    tracing::error!("Failed to add sequencer: {:?}", err);
                    return Err(err.into());
                }
            }
        };

        Ok(RegisterResponse { success: true })
    }
}
