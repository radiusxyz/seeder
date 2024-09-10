use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    rpc::prelude::*,
    state::AppState,
    types::prelude::{Platform, SequencingInfoPayload, SequencingInfosModel, ServiceProvider},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfosResponse {
    sequencing_infos: Vec<((Platform, ServiceProvider), SequencingInfoPayload)>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencingInfosResponse, RpcError> {
        let sequencing_infos = SequencingInfosModel::get()?
            .sequencing_infos()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();

        Ok(GetSequencingInfosResponse { sequencing_infos })
    }
}
