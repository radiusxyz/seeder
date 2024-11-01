use crate::{rpc::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    external_rpc_url: String,
    cluster_rpc_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSequencer {
    message: RegisterSequencerMessage,
    signature: Signature,
}

impl RegisterSequencer {
    pub const METHOD_NAME: &'static str = "register_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Self>()?;

        // Verify the message.
        parameter.signature.verify_message(
            parameter.message.platform.try_into()?,
            &parameter.message,
            &parameter.message.address,
        )?;

        tracing::info!("Register sequencer: {:?}", parameter.message.address);

        match parameter.message.platform {
            Platform::Ethereum => {
                let sequencing_key = (
                    parameter.message.platform,
                    parameter.message.service_provider,
                );
                let publisher = context.get_publisher(&sequencing_key).await?;
                let block_number = publisher.get_block_number().await?;

                let sequencer_list = publisher
                    .get_sequencer_list(&parameter.message.cluster_id, block_number)
                    .await?;

                // check if the sequencer is registered in the contract
                sequencer_list
                    .iter()
                    .find(|&&address| parameter.message.address == address)
                    .ok_or(Error::NotRegisteredInContract)?;
            }
            Platform::Local => {}
        }

        // health check
        health_check(&parameter.message.external_rpc_url).await?;

        let sequencer_node_info = SequencerNodeInfo::new(
            parameter.message.address,
            parameter.message.external_rpc_url,
            parameter.message.cluster_rpc_url,
        );

        SequencerNodeInfo::put(&sequencer_node_info, sequencer_node_info.address())?;

        Ok(())
    }
}
