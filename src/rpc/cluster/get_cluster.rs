use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::publisher::Publisher,
    signature::{ChainType, Signature},
};

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetClusterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
    cluster_id: ClusterId,
}

impl std::fmt::Display for GetClusterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCluster {
    signature: Signature,
    message: GetClusterMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterResponse {
    cluster: ClusterModel,
}

impl GetCluster {
    pub const METHOD_NAME: &'static str = "get_cluster";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<Publisher>,
    ) -> Result<GetClusterResponse, RpcError> {
        let parameter = parameter.parse::<GetCluster>()?;

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

        let cluster_model = match parameter.message.sequencing_function_type {
            SequencingFunctionType::Liveness => ClusterModel::Liveness(LivenessClusterModel::get(
                &platform,
                &parameter.message.service_type,
                &parameter.message.cluster_id,
            )?),
            SequencingFunctionType::Validation => {
                ClusterModel::Validation(ValidationClusterModel::get(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?)
            }
        };

        Ok(GetClusterResponse {
            cluster: cluster_model,
        })
    }
}
