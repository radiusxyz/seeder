use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfosResponse {
    sequencing_infos: Vec<((Platform, ServiceProvider), SequencingInfoPayload)>,
}

impl RpcParameter<AppState> for GetSequencingInfos {
    type Response = GetSequencingInfosResponse;

    fn method() -> &'static str {
        "get_sequencing_infos"
    }

    async fn handler(self, _context: AppState) -> Result<Self::Response, RpcError> {
        let sequencing_info_list = SequencingInfoList::get()?;

        let sequencing_infos: Vec<((Platform, ServiceProvider), SequencingInfoPayload)> =
            sequencing_info_list
                .iter()
                .filter_map(|(platform, service_provider)| {
                    if let Ok(payload) = SequencingInfoPayload::get(*platform, *service_provider) {
                        Some(((*platform, *service_provider), payload))
                    } else {
                        None
                    }
                })
                .collect();

        Ok(GetSequencingInfosResponse { sequencing_infos })
    }
}
