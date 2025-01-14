use crate::{rpc::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSequencer {
    message: RegisterSequencerMessage,
    signature: Signature,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    external_rpc_url: String,
    cluster_rpc_url: String,
}

impl RpcParameter<AppState> for RegisterSequencer {
    type Response = ();

    fn method() -> &'static str {
        "register_sequencer"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        // Verify the message.
        // parameter.signature.verify_message(
        //     parameter.message.platform.into(),
        //     &parameter.message,
        //     &parameter.message.address,
        // )?;

        tracing::info!(
            "Register sequencer - address: {:?}",
            self.message.address.as_hex_string()
        );

        match self.message.platform {
            Platform::Ethereum => {
                let liveness_client: liveness::radius::LivenessClient = context
                    .get_liveness_client(self.message.platform, self.message.service_provider)
                    .await?;
                let block_number = liveness_client
                    .publisher()
                    .get_block_number()
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?;

                let sequencer_list = liveness_client
                    .publisher()
                    .get_sequencer_list(&self.message.cluster_id, block_number)
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?;

                // check if the sequencer is registered in the contract
                sequencer_list
                    .iter()
                    .find(|&&address| self.message.address == address)
                    .ok_or(Error::NotRegisteredInContract)?;
            }
            Platform::Local => return Err(Error::UnsupportedPlatform.into()),
        }

        // health check
        health_check(&self.message.external_rpc_url).await?;

        let sequencer_node_info = SequencerNodeInfo::new(
            self.message.address,
            self.message.external_rpc_url,
            self.message.cluster_rpc_url,
        );

        SequencerNodeInfo::put(&sequencer_node_info, sequencer_node_info.address())?;

        Ok(())
    }
}
