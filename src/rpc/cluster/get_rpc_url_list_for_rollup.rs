use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::{publisher::Publisher, types::Address},
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{
    models::prelude::*,
    rpc::{methods::serialize_to_bincode, prelude::*},
    sequencer_types::prelude::*,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlListForRollupMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
    block_height: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListForRollup {
    signature: Signature,
    message: GetRpcUrlListForRollupMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListForRollupResponse {
    pub rpc_url_list: Vec<(Address, IpAddress)>,
    pub block_height: u64,
}

impl GetRpcUrlListForRollup {
    pub const METHOD_NAME: &'static str = "get_rpc_url_list_for_rollup";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<Publisher>,
    ) -> Result<GetRpcUrlListForRollupResponse, RpcError> {
        let parameter = parameter.parse::<GetRpcUrlListForRollup>()?;

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

        let sequencer_list = context
            .get_sequencer_list(
                &parameter.message.cluster_id,
                parameter.message.block_height,
            )
            .await?;

        let rpc_url_list = sequencer_list
            .into_iter()
            .filter_map(|address| {
                SequencerModel::get(&address)
                    .ok()
                    .and_then(|sequencer_model| {
                        sequencer_model.rpc_url.map(|rpc_url| (address, rpc_url))
                    })
            })
            .collect();

        Ok(GetRpcUrlListForRollupResponse {
            rpc_url_list,
            block_height: parameter.message.block_height,
        })
    }
}
