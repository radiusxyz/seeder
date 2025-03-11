use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfos {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfosResponse {
    liveness_infos: Vec<((Platform, ServiceProvider), LivenessInfoPayload)>,
}

impl RpcParameter<AppState> for GetLivenessInfos {
    type Response = GetLivenessInfosResponse;

    fn method() -> &'static str {
        "get_liveness_infos"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let liveness_info_list = LivenessInfoList::get()?;

        let liveness_infos: Vec<((Platform, ServiceProvider), LivenessInfoPayload)> =
            liveness_info_list
                .iter()
                .filter_map(|(platform, service_provider)| {
                    if let Ok(payload) = LivenessInfoPayload::get(*platform, *service_provider) {
                        Some(((*platform, *service_provider), payload))
                    } else {
                        None
                    }
                })
                .collect();

        Ok(GetLivenessInfosResponse { liveness_infos })
    }
}
