use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::{publisher::Publisher, types::hex},
    signature::{ChainType, Signature},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::Error,
    models::prelude::SequencerModel,
    rpc::prelude::*,
    sequencer_types::prelude::{Address, ClusterId, IpAddress},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    cluster_id: ClusterId,
}

impl std::fmt::Display for GetRpcUrlMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        if !context
            .is_registered(parameter.message.cluster_id.clone())
            .await?
        {
            tracing::error!("Not registered on the Liveness contract.");

            // return Err(Error::Publisher(
            //     radius_sequencer_sdk::liveness::publisher::PublisherError::IsRegistered(
            //         alloy_contract::error::Error::UnknownFunction(
            //             "Not registered on the Liveness contract.".to_string(),
            //         ),
            //     ),
            // )
            // .into());
        }

        let address = Address::from(hex::encode(&parameter.message.address));

        let sequencer_model = SequencerModel::get(address)?;

        Ok(GetRpcUrlResponse {
            rpc_url: sequencer_model.rpc_url,
        })
    }
}
