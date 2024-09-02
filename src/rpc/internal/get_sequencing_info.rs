use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    rpc::prelude::*,
    sequencer_types::prelude::{SequencingInfoKey, SequencingInfoPayload},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfo {
    pub sequencing_info_key: SequencingInfoKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfoResponse {
    sequencing_info_payload: SequencingInfoPayload,
}

impl GetSequencingInfo {
    pub const METHOD_NAME: &'static str = "get_sequencing_info";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetSequencingInfoResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencingInfo>()?;
        let sequencing_info_payload = context.get_sequencing_info(parameter.sequencing_info_key)?;

        Ok(GetSequencingInfoResponse {
            sequencing_info_payload,
        })
    }
}
