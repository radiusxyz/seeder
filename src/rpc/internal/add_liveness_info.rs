use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddLivenessInfo {
    pub platform: Platform,
    pub liveness_service_provider: LivenessServiceProvider,
    pub payload: LivenessInfoPayload,
}

impl RpcParameter<AppState> for AddLivenessInfo {
    type Response = ();

    fn method() -> &'static str {
        "add_liveness_info"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        tracing::info!(
            "Add liveness info - platform: {:?}, service provider: {:?}, payload: {:?}",
            self.platform,
            self.liveness_service_provider,
            self.payload
        );

        // Save `LivenessClient` metadata.
        let mut liveness_info_list = LivenessInfoList::get_mut_or(LivenessInfoList::default)?;
        liveness_info_list.insert(self.platform, self.liveness_service_provider);
        liveness_info_list.update()?;

        LivenessInfoPayload::put(&self.payload, self.platform, self.liveness_service_provider)?;

        match &self.payload {
            LivenessInfoPayload::Ethereum(liveness_info) => {
                liveness::radius::LivenessClient::initialize(
                    context.clone(),
                    self.platform,
                    self.liveness_service_provider,
                    liveness_info.clone(),
                );
            }
            LivenessInfoPayload::Local(_liveness_info) => {
                todo!("Implement 'LivenessClient' for local ordering.");
            }
        }

        Ok(())
    }
}
