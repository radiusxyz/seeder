use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetSequencerRpcUrlListAtBlockHeigthMessage {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    block_height: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeight {
    message: GetSequencerRpcUrlListAtBlockHeigthMessage,
    signature: Signature,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeighResponse {
    pub rpc_url_list: Vec<(String, Option<String>)>,
    pub block_height: u64,
}

impl GetSequencerRpcUrlListAtBlockHeight {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list_at_block_height";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListAtBlockHeighResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        tracing::info!(
            "get_sequencer_rpc_url_list_for_rollup: {:?}",
            parameter.message.cluster_id
        );

        let sequencing_key = (
            parameter.message.platform,
            parameter.message.service_provider,
        );

        let publisher = context.get_publisher(&sequencing_key).await?;
        let block_number = publisher.get_block_number().await?;

        let sequencer_list = publisher
            .get_sequencer_list(&parameter.message.cluster_id, block_number)
            .await?;

        let rpc_url_list: Vec<(String, Option<String>)> = sequencer_list
            .into_iter()
            .map(|address| {
                let address = Address::from(address.as_slice().to_vec());
                let rpc_url = SequencerNodeInfo::get(&address)
                    .map(|node_info| node_info.into_rpc_url())
                    .ok();

                (address.as_hex_string(), rpc_url)
            })
            .collect();

        Ok(GetSequencerRpcUrlListAtBlockHeighResponse {
            rpc_url_list,
            block_height: parameter.message.block_height,
        })
    }
}
