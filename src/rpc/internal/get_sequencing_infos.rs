use std::{collections::BTreeMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    models::prelude::SequencingInfosModel, rpc::prelude::*,
    sequencer_types::prelude::SequencingInfoPayload, state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos {}

// TODO:
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfosResponse {
    sequencing_infos: BTreeMap<String, SequencingInfoPayload>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencingInfosResponse, RpcError> {
        let sequencing_infos_model = SequencingInfosModel::get()?;
        let sequencing_infos = sequencing_infos_model.sequencing_infos().clone();

        Ok(GetSequencingInfosResponse { sequencing_infos })
    }
}
