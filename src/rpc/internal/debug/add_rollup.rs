use crate::{rpc::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddRollup {
    pub message: AddRollupMessage,
    pub signature: Signature,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddRollupMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    rpc_url: String,
}

impl RpcParameter<AppState> for AddRollup {
    type Response = ();

    fn method() -> &'static str {
        "add_rollup"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        // verify siganture
        self.signature.verify_message(
            self.message.platform.into(),
            &self.message,
            &self.message.address,
        )?;

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
                    .find(|&address| self.message.address == address)
                    .ok_or(Error::NotRegisteredInContract)?;
            }
            Platform::Local => {}
        }

        // health check
        health_check(&self.message.rpc_url).await?;

        let rollup_node_info = RollupNodeInfo::new(self.message.address, self.message.rpc_url);
        RollupNodeInfo::put(&rollup_node_info, rollup_node_info.address())?;

        Ok(())
    }
}
