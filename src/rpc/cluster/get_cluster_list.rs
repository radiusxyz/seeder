use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::publisher::Publisher,
    signature::{ChainType, Signature},
};

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetClusterListMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
}

impl std::fmt::Display for GetClusterListMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterList {
    signature: Signature,
    message: GetClusterListMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterListResponse {
    cluster_list: Vec<ClusterModel>,
}

impl GetClusterList {
    pub const METHOD_NAME: &'static str = "get_cluster_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<Publisher>,
    ) -> Result<GetClusterListResponse, RpcError> {
        let parameter = parameter.parse::<GetClusterList>()?;

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        let platform = match parameter.message.chain_type {
            ChainType::Ethereum => PlatForm::Ethereum,
            _ => PlatForm::Local,
        };

        let cluster_id_list_model = ClusterIdListModel::get(
            &platform,
            &parameter.message.sequencing_function_type,
            &parameter.message.service_type,
        )?;

        let cluster_list = cluster_id_list_model
            .cluster_id_list()
            .as_ref()
            .iter()
            .map(|cluster_id| {
                Ok(match parameter.message.sequencing_function_type {
                    SequencingFunctionType::Liveness => {
                        ClusterModel::Liveness(LivenessClusterModel::get(
                            &platform,
                            &parameter.message.service_type,
                            cluster_id,
                        )?)
                    }
                    SequencingFunctionType::Validation => {
                        ClusterModel::Validation(ValidationClusterModel::get(
                            &platform,
                            &parameter.message.service_type,
                            cluster_id,
                        )?)
                    }
                })
            })
            .collect::<Result<Vec<ClusterModel>, DbError>>()?;

        Ok(GetClusterListResponse { cluster_list })
    }
}
