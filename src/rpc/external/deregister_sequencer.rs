use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterSequencer {
    message: DeregisterSequencerMessage,
    signature: Signature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeregisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
}

impl RpcParameter<AppState> for DeregisterSequencer {
    type Response = ();

    fn method() -> &'static str {
        "deregister_sequencer"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        // Verify the message.
        // self.signature.verify_message(
        //     self.message.platform.into(),
        //     &self.message,
        //     &self.message.address,
        // )?;

        tracing::info!(
            "Deregister sequencer: {:?}",
            self.message.address.as_hex_string()
        );

        match self.message.platform {
            Platform::Ethereum => {
                let liveness_client: liveness::radius::LivenessClient = context
                    .get_liveness_client(self.message.platform, self.message.service_provider)
                    .await?;
                let block_margin = liveness_client
                    .publisher()
                    .get_block_margin()
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?
                    .try_into()?;
                let block_number = liveness_client
                    .publisher()
                    .get_block_number()
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?
                    .wrapping_sub(block_margin);
                let sequencer_list = liveness_client
                    .publisher()
                    .get_sequencer_list(&self.message.cluster_id, block_number)
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?;

                // check if the sequencer is deregistered from the contract
                sequencer_list
                    .iter()
                    .find(|&&address| self.message.address == address)
                    .map_or(Ok(()), |_| Err(Error::NotDeregisteredFromContract))?;
            }
            Platform::Local => return Err(Error::UnsupportedPlatform.into()),
        }

        SequencerNodeInfo::delete(&self.message.address)?;

        Ok(())
    }
}
