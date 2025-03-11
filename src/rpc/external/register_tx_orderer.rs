use crate::{rpc::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterTxOrderer {
    message: RegisterTxOrdererMessage,
    signature: Signature,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterTxOrdererMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    tx_orderer_address: Address,
    external_rpc_url: String,
    cluster_rpc_url: String,
}

impl RpcParameter<AppState> for RegisterTxOrderer {
    type Response = ();

    fn method() -> &'static str {
        "register_tx_orderer"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        // Verify the message.
        // parameter.signature.verify_message(
        //     parameter.message.platform.into(),
        //     &parameter.message,
        //     &parameter.message.tx_orderer_address,
        // )?;

        tracing::info!(
            "Register tx_orderer - address: {:?}",
            self.message.tx_orderer_address.as_hex_string()
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

                let tx_orderer_list = liveness_client
                    .publisher()
                    .get_tx_orderer_list(&self.message.cluster_id, block_number)
                    .await
                    .map_err(|error| Error::LivenessClient(error.into()))?;

                // check if the tx_orderer is registered in the contract
                tx_orderer_list
                    .iter()
                    .find(|&&address| self.message.tx_orderer_address == address)
                    .ok_or(Error::NotRegisteredInContract)?;
            }
            Platform::Local => return Err(Error::UnsupportedPlatform.into()),
        }

        // health check
        health_check(&self.message.external_rpc_url).await?;

        let tx_orderer_node_info = TxOrdererRpcInfo::new(
            self.message.tx_orderer_address,
            self.message.external_rpc_url,
            self.message.cluster_rpc_url,
        );

        TxOrdererRpcInfo::put(
            &tx_orderer_node_info,
            tx_orderer_node_info.tx_orderer_address(),
        )?;

        Ok(())
    }
}
