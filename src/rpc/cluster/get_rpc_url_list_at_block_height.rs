use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::types::Address,
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{
    models::prelude::*,
    rpc::{methods::serialize_to_bincode, prelude::*},
    sequencer_types::prelude::*,
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlListAtBlockHeigthMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
    block_height: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListAtBlockHeight {
    signature: Signature,
    message: GetRpcUrlListAtBlockHeigthMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListAtBlockHeighResponse {
    pub rpc_url_list: Vec<(Address, Option<IpAddress>)>,
    pub block_height: u64,
}

impl GetRpcUrlListAtBlockHeight {
    pub const METHOD_NAME: &'static str = "get_rpc_url_list_at_block_height";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetRpcUrlListAtBlockHeighResponse, RpcError> {
        let parameter = parameter.parse::<GetRpcUrlListAtBlockHeight>()?;

        info!(
            "get_rpc_url_list_for_rollup: {:?}",
            parameter.message.cluster_id
        );

        // verify siganture
        parameter.signature.verify_signature(
            serialize_to_bincode(&parameter.message)?.as_slice(),
            parameter.message.address.as_slice(),
            parameter.message.chain_type,
        )?;

        let sequencing_key =
            ClusterInfoModel::get(&parameter.message.cluster_id)?.sequencing_info_key();

        let publisher = context.get_publisher(sequencing_key)?;
        let sequencer_list = publisher
            .get_sequencer_list(
                &parameter.message.cluster_id,
                parameter.message.block_height,
            )
            .await?;

        let cluster_info = context.get_cluster_info(&parameter.message.cluster_id)?;
        let sequencer_rpc_url_list = cluster_info.sequencer_rpc_url_list();

        let rpc_url_list = sequencer_list
            .into_iter()
            .filter_map(|address| {
                sequencer_rpc_url_list
                    .iter()
                    .find(|(sequencer_address, _)| sequencer_address == &address)
                    .cloned()
            })
            .collect();

        Ok(GetRpcUrlListAtBlockHeighResponse {
            rpc_url_list,
            block_height: parameter.message.block_height,
        })
    }
}
