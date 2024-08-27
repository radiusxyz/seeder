use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::publisher::Publisher,
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlListMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    cluster_id: ClusterId,
    sequencer_address_list: Vec<Address>,
}

impl std::fmt::Display for GetRpcUrlListMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlList {
    signature: Signature,
    message: GetRpcUrlListMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListResponse {
    pub rpc_url_list: Vec<(Address, IpAddress)>,
}

impl GetRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<Publisher>,
    ) -> Result<GetRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<GetRpcUrlList>()?;

        info!("get_rpc_url_list: {:?}", parameter.message.cluster_id);

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        let rpc_url_list = parameter
            .message
            .sequencer_address_list
            .into_iter()
            .filter_map(|address| {
                SequencerModel::get(&address)
                    .ok()
                    .and_then(|sequencer| sequencer.rpc_url.map(|rpc_url| (address, rpc_url)))
            })
            .collect();

        Ok(GetRpcUrlListResponse { rpc_url_list })
    }
}
