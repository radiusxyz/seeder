use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::types::Address,
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    models::prelude::SequencerModel,
    rpc::prelude::*,
    sequencer_types::prelude::{ClusterId, IpAddress},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetSequencerRpcUrlMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrl {
    signature: Signature,
    message: GetSequencerRpcUrlMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlResponse {
    pub rpc_url: Option<IpAddress>,
}

impl GetSequencerRpcUrl {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencerRpcUrl>()?;

        info!("get_sequencer_rpc_url: {:?}", parameter.message.address);

        // // verify siganture
        // parameter.signature.verify_signature(
        //     serialize_to_bincode(&parameter.message)?.as_slice(),
        //     parameter.message.address.as_slice(),
        //     parameter.message.chain_type,
        // )?;

        // let cluster_info = context.get_cluster_info(&parameter.message.cluster_id)?;
        let rpc_url = SequencerModel::get(&parameter.message.address)?.rpc_url;

        Ok(GetSequencerRpcUrlResponse { rpc_url })
    }
}
