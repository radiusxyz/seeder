use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::{publisher::Publisher, types::hex},
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlListForRollupMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    cluster_id: ClusterId,
    block_height: u64,
}

impl std::fmt::Display for GetRpcUrlListForRollupMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
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
                let address = Address::from(hex::encode(address));
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
