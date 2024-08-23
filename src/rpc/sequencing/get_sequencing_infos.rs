use std::{collections::HashMap, sync::Arc};

use radius_sequencer_sdk::json_rpc::{types::RpcParameter, RpcError};
pub use serde::{Deserialize, Serialize};

use crate::{models::prelude::SequencingInfoModel, sequencer_types::prelude::SequencingInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos {}

// TODO:
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfosResponse {
    sequencing_infos: HashMap<String, SequencingInfo>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        _context: Arc<()>,
    ) -> Result<GetLivenessInfosResponse, RpcError> {
        let sequencing_info_model = SequencingInfoModel::get()?;

        println!(
            "sequencing_infos: {:?}",
            sequencing_info_model.sequencing_infos()
        );

        let sequencing_infos = sequencing_info_model
            .sequencing_infos()
            .clone()
            .into_iter()
            .map(|(sequencing_info_key, sequencing_info)| {
                (sequencing_info_key.to_string(), sequencing_info)
            })
            .collect();

        Ok(GetLivenessInfosResponse { sequencing_infos })
    }
}
