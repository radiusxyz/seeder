use std::sync::Arc;

use radius_sequencer_sdk::liveness::publisher::Publisher;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    models::prelude::SequencingInfosModel,
    rpc::prelude::*,
    sequencer_types::prelude::{
        sequencing_key, LivenessEthereum, LivenessLocal, Platform, SequencingInfoPayload,
        ServiceProvider,
    },
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "SequencingInfo")]
pub struct AddSequencingInfo {
    pub platform: Platform,
    pub service_provider: ServiceProvider,
    pub payload: SequencingInfoPayload,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SequencingInfo {
    platform: Platform,
    service_provider: ServiceProvider,
    payload: serde_json::Value,
}

impl TryFrom<SequencingInfo> for AddSequencingInfo {
    type Error = Error;

    fn try_from(value: SequencingInfo) -> Result<Self, Self::Error> {
        match value.platform {
            Platform::Ethereum => {
                let payload: LivenessEthereum =
                    serde_json::from_value(value.payload).map_err(Error::Deserialize)?;

                Ok(Self {
                    platform: value.platform,
                    service_provider: value.service_provider,
                    payload: SequencingInfoPayload::Ethereum(payload),
                })
            }
            Platform::Local => {
                let payload: LivenessLocal =
                    serde_json::from_value(value.payload).map_err(Error::Deserialize)?;

                Ok(Self {
                    platform: value.platform,
                    service_provider: value.service_provider,
                    payload: SequencingInfoPayload::Local(payload),
                })
            }
        }
    }
}

impl AddSequencingInfo {
    pub const METHOD_NAME: &'static str = "add_sequencing_info";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<AddSequencingInfo>()?;

        tracing::info!("add_sequencing_info: {:?}", parameter);

        let mut sequencing_infos = SequencingInfosModel::get_mut()?;

        let sequencing_key = sequencing_key(parameter.platform, parameter.service_provider);
        // Todo: change key
        if sequencing_infos
            .sequencing_infos()
            .get(&sequencing_key)
            .is_some()
        {
            return Err(Error::ExistSequencingInfo.into());
        }

        sequencing_infos.insert(sequencing_key.clone(), parameter.payload.clone());
        sequencing_infos.update()?;

        match parameter.payload {
            SequencingInfoPayload::Ethereum(payload) => {
                if context.get_publisher(&sequencing_key).await.is_ok() {
                    return Err(Error::PublisherAlreadyExists.into());
                }

                let publisher = Publisher::new(
                    payload.rpc_url,
                    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                    payload.contract_address,
                )?;

                // add publisher to app state
                context
                    .add_publisher(sequencing_key, Arc::new(publisher))
                    .await;
            }
            _ => {}
        }

        Ok(())
    }
}
