use crate::rpc::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeregisterSequencerMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterSequencer {
    message: DeregisterSequencerMessage,
    signature: Signature,
}

impl DeregisterSequencer {
    pub const METHOD_NAME: &'static str = "deregister_sequencer";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Self>()?;

        // // verify siganture
        // parameter.signature.verify_message(
        //     parameter.message.platform.into(),
        //     &parameter.message,
        //     parameter.message.address.as_ref(),
        // )?;

        let sequencing_key = (
            parameter.message.platform,
            parameter.message.service_provider,
        );

        let sequencing_info = SequencingInfosModel::get()?;
        let sequencing_info_payload = sequencing_info
            .sequencing_infos()
            .get(&sequencing_key)
            .ok_or(Error::FailedToGetSequencingInfo)?;

        match sequencing_info_payload {
            SequencingInfoPayload::Ethereum(_payload) => {
                let publisher = context.get_publisher(&sequencing_key).await?;

                let block_number = publisher.get_block_number().await?;
                let sequencer_list = publisher
                    .get_sequencer_list(&parameter.message.cluster_id, block_number)
                    .await?;

                // check if the sequencer is deregistered from the contract
                sequencer_list
                    .iter()
                    .find(|&&address| parameter.message.address == address)
                    .map_or(Ok(()), |_| Err(Error::NotDeregisteredFromContract))?;
            }
            _ => {}
        }

        SequencerNodeInfoModel::delete(&parameter.message.address)?;

        Ok(())
    }
}
