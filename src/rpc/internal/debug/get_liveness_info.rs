use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfo {
    pub platform: Platform,
    pub liveness_service_provider: LivenessServiceProvider,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfoResponse {
    liveness_info_payload: LivenessInfoPayload,
}

impl RpcParameter<AppState> for GetLivenessInfo {
    type Response = GetLivenessInfoResponse;

    fn method() -> &'static str {
        "get_liveness_info"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let liveness_info_payload =
            LivenessInfoPayload::get(self.platform, self.liveness_service_provider)?;

        Ok(GetLivenessInfoResponse {
            liveness_info_payload,
        })
    }
}
