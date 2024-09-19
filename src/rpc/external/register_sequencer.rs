use crate::{rpc::prelude::*, util::health_check};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RegisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    rpc_url: String,
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

        // // verify siganture
        // parameter.signature.verify_message(
        //     parameter.message.platform.try_into()?,
        //     &parameter.message,
        //     &parameter.message.address,
        // )?;

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
        health_check(&parameter.message.rpc_url).await?;

        SequencerNodeInfoModel::put(&parameter.message.address, &parameter.message.rpc_url)?;

        Ok(())
    }
}
