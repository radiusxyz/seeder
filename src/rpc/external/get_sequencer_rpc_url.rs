use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{rpc::prelude::*, state::AppState, types::prelude::SequencerNodeInfoModel};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrl {
    address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlResponse {
    pub sequencer_rpc_url: Option<String>,
}

impl GetSequencerRpcUrl {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencerRpcUrl>()?;

        let sequencer_rpc_url = SequencerNodeInfoModel::get(&parameter.address)?.rpc_url;

        Ok(GetSequencerRpcUrlResponse { sequencer_rpc_url })
    }
}
