use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::{publisher::Publisher, types::Address},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::Error,
    models::prelude::SequencerModel,
    rpc::{methods::serialize_to_bincode, prelude::*},
    sequencer_types::prelude::{ClusterId, IpAddress},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlMessage {
    address: Address,
    chain_type: ChainType,
    cluster_id: ClusterId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrl {
    signature: Signature,
    message: GetRpcUrlMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlResponse {
    pub rpc_url: Option<IpAddress>,
}

impl GetRpcUrl {
    pub const METHOD_NAME: &'static str = "get_rpc_url";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<Publisher>,
    ) -> Result<GetRpcUrlResponse, RpcError> {
        let parameter = parameter.parse::<GetRpcUrl>()?;

        info!("get_rpc_url: {:?}", parameter.message.address);

        // verify siganture
        parameter.signature.verify_signature(
            serialize_to_bincode(&parameter.message)?.as_slice(),
            parameter.message.address.as_slice(),
            parameter.message.chain_type,
        )?;

        let block_number = context.get_block_number().await?;
        let sequencer_list = context
            .get_sequencer_list(&parameter.message.cluster_id, block_number)
            .await?;

        sequencer_list
            .iter()
            .find(|&address| address.as_slice() == parameter.message.address)
            .ok_or(Error::UnRegistered)?;

        let sequencer_model = SequencerModel::get(&parameter.message.address)?;

        Ok(GetRpcUrlResponse {
            rpc_url: sequencer_model.rpc_url,
        })
    }
}
