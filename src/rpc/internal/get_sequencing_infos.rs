use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{rpc::prelude::*, sequencer_types::prelude::SequencingInfoPayload, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos {}

// TODO:
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfosResponse {
    sequencing_infos: HashMap<String, SequencingInfoPayload>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<GetSequencingInfosResponse, RpcError> {
        let sequencing_infos = context
            .sequencing_infos()
            .as_ref()
            .iter()
            .map(|(sequencing_info_key, sequencing_info)| {
                (sequencing_info_key.to_string(), sequencing_info.clone())
            })
            .collect();

        Ok(GetSequencingInfosResponse { sequencing_infos })
    }
}
