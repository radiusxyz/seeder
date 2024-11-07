use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddSequencingInfo {
    pub platform: Platform,
    pub service_provider: ServiceProvider,
    pub payload: SequencingInfoPayload,
}

impl AddSequencingInfo {
    pub const METHOD_NAME: &'static str = "add_sequencing_info";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Self>()?;

        tracing::info!(
            "Add sequencing info - platform: {:?} / service_provider: {:?}",
            parameter.platform,
            parameter.service_provider
        );

        // Save `LivenessClient` metadata.
        let mut sequencing_info_list = SequencingInfoList::get_mut_or(SequencingInfoList::default)?;

        let sequencing_key = (parameter.platform, parameter.service_provider);

        sequencing_info_list.insert(parameter.platform, parameter.service_provider);
        sequencing_info_list.update()?;

        SequencingInfoPayload::put(
            &parameter.payload,
            parameter.platform,
            parameter.service_provider,
        )?;

        match &parameter.payload {
            SequencingInfoPayload::Ethereum(payload) => {
                if context.get_publisher(&sequencing_key).await.is_ok() {
                    tracing::error!("Publisher already exists: {:?}", sequencing_key);
                    return Err(Error::PublisherAlreadyExists.into());
                }

                let publisher = Publisher::new(
                    payload.liveness_rpc_url.clone(),
                    // TODO(jaemin): remove this hard-coded value
                    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                    payload.contract_address.clone(),
                )?;

                // add publisher to app state
                context
                    .add_publisher(sequencing_key, Arc::new(publisher))
                    .await;
            }
            SequencingInfoPayload::Local(_payload) => {
                // liveness::local::LivenessClient::new()?;
                todo!("Implement 'LivenessClient' for local sequencing.");
            }
        }

        Ok(())
    }
}
