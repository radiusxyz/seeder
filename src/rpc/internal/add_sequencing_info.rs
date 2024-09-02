use std::sync::Arc;

use radius_sequencer_sdk::liveness::publisher::Publisher;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::prelude::{SequencingInfoKeyListModel, SequencingInfoModel},
    rpc::prelude::*,
    sequencer_types::prelude::{
        ContractAddress, IpAddress, Platform, SequencingCondition, SequencingFunctionType,
        SequencingInfoKey, SequencingInfoPayload, ServiceProvider,
    },
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddSequencingInfo {
    pub platform: Platform,                               // Local / Ethereum
    pub sequencing_function_type: SequencingFunctionType, // Liveness / Validation
    pub service_provider: ServiceProvider,                // Radius / EigenLayer

    pub provider_rpc_url: Option<IpAddress>,
    pub provider_websocket_url: Option<IpAddress>,

    pub contract_address: Option<ContractAddress>,
}

impl AddSequencingInfo {
    pub const METHOD_NAME: &'static str = "add_sequencing_info";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<AddSequencingInfo>()?;

        let sequencing_info_key = SequencingInfoKey::new(
            parameter.platform,
            parameter.sequencing_function_type,
            parameter.service_provider,
        );
        let sequencing_info_payload = SequencingInfoPayload::new(
            parameter.provider_rpc_url.clone(),
            parameter.provider_websocket_url,
            parameter.contract_address.clone(),
        );

        if matches!(
            SequencingCondition::from(sequencing_info_key),
            SequencingCondition::EthereumLivenessRadius
        ) {
            if context.get_publisher(sequencing_info_key).is_ok() {
                return Err(Error::PublisherAlreadyExists.into());
            }

            let publisher = Publisher::new(
                parameter.provider_rpc_url.unwrap(),
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                parameter.contract_address.unwrap(),
            )?;

            // add publisher to app state
            context.add_publisher(sequencing_info_key, Arc::new(publisher));
        }

        let mut sequencing_info_key_list_model = match SequencingInfoKeyListModel::get_mut() {
            Ok(sequencing_info_key_list_model) => sequencing_info_key_list_model,
            Err(err) => {
                if err.is_none_type() {
                    SequencingInfoKeyListModel::default().put()?;
                    SequencingInfoKeyListModel::get_mut()?
                } else {
                    return Err(err.into());
                }
            }
        };

        sequencing_info_key_list_model.add_sequencing_info_key(sequencing_info_key);
        sequencing_info_key_list_model.update()?;

        // Add sequencing info to db
        SequencingInfoModel::new(sequencing_info_key, sequencing_info_payload.clone())
            .put(sequencing_info_key)?;
        // Add sequencing info to app state
        context.add_sequencing_info(sequencing_info_key, sequencing_info_payload);

        Ok(())
    }
}
