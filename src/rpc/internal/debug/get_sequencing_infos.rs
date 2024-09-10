use std::{collections::BTreeMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    rpc::prelude::*,
    state::AppState,
    types::prelude::{SequencingInfoPayload, SequencingInfosModel},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos;

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
        let sequencing_infos = SequencingInfosModel::get()?.sequencing_infos().clone();

        Ok(GetSequencingInfosResponse { sequencing_infos })
    }
}
