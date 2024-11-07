use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeight {
    platform: Platform,
    service_provider: ServiceProvider,
    cluster_id: String,
    address: Address,
    block_number: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListAtBlockHeighResponse {
    pub sequencer_rpc_url_list: Vec<(String, Option<(String, String)>)>,
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
            "Get sequencer rpc url list for rollup - rollup id: {:?} / block number: {:?}",
            parameter.cluster_id,
            parameter.block_number
        );

        let sequencing_key = (parameter.platform, parameter.service_provider);

        let publisher = context.get_publisher(&sequencing_key).await?;
        // let block_number = publisher.get_block_number().await?;

        let sequencer_address_list = publisher
            .get_sequencer_list(&parameter.cluster_id, parameter.block_number)
            .await?;

        let sequencer_rpc_url_list: Vec<(String, Option<(String, String)>)> =
            sequencer_address_list
                .into_iter()
                .map(|address| {
                    let address = Address::from(address.as_slice().to_vec());
                    let sequencer_rpc_url = SequencerNodeInfo::get(&address)
                        .map(|node_info| {
                            (
                                node_info.external_rpc_url().to_owned(),
                                node_info.cluster_rpc_url().to_owned(),
                            )
                        })
                        .ok();

                    (address.as_hex_string(), sequencer_rpc_url)
                })
                .collect();

        Ok(GetSequencerRpcUrlListAtBlockHeighResponse {
            sequencer_rpc_url_list,
            block_height: parameter.block_number,
        })
    }
}
