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
pub struct GetSequencerRpcUrlListAtBlockHeightResponse {
    pub sequencer_rpc_url_list: Vec<(String, Option<(String, String)>)>,
    pub block_height: u64,
}

impl RpcParameter<AppState> for GetSequencerRpcUrlListAtBlockHeight {
    type Response = GetSequencerRpcUrlListAtBlockHeightResponse;

    fn method() -> &'static str {
        "get_sequencer_rpc_url_list_at_block_height"
    }

    async fn handler(self, context: AppState) -> Result<Self::Response, RpcError> {
        tracing::info!(
            "Get sequencer rpc url list for rollup - rollup id: {:?} / block number: {:?}",
            self.cluster_id,
            self.block_number
        );

        let liveness_client: liveness::radius::LivenessClient = context
            .get_liveness_client(self.platform, self.service_provider)
            .await?;
        // let block_number = publisher.get_block_number().await?;

        let sequencer_address_list = liveness_client
            .publisher()
            .get_sequencer_list(&self.cluster_id, self.block_number)
            .await
            .map_err(|error| Error::LivenessClient(error.into()))?;

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

        Ok(GetSequencerRpcUrlListAtBlockHeightResponse {
            sequencer_rpc_url_list,
            block_height: self.block_number,
        })
    }
}
