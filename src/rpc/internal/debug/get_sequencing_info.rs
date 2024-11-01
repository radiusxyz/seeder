use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfo {
    pub platform: Platform,
    pub service_provider: ServiceProvider,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfoResponse {
    sequencing_info_payload: SequencingInfoPayload,
}

impl GetSequencingInfo {
    pub const METHOD_NAME: &'static str = "get_sequencing_info";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencingInfoResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencingInfo>()?;

        let sequencing_info_payload =
            SequencingInfoPayload::get(parameter.platform, parameter.service_provider)?;

        Ok(GetSequencingInfoResponse {
            sequencing_info_payload,
        })
    }
}
