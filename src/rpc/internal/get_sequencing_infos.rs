use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    rpc::prelude::*,
    sequencer_types::prelude::{SequencingInfoKey, SequencingInfoPayload},
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos {}

// TODO:
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfosResponse {
    sequencing_infos: HashMap<SequencingInfoKey, SequencingInfoPayload>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetLivenessInfosResponse, RpcError> {
        let sequencing_infos = context.sequencing_infos().as_ref().clone();

        Ok(GetLivenessInfosResponse { sequencing_infos })
    }
}
