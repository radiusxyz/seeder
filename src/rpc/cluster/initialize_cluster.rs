use std::sync::Arc;

use prelude::*;
use radius_sequencer_sdk::{
    liveness::publisher::Publisher,
    signature::{ChainType, Signature},
};

use crate::{models::prelude::*, rpc::prelude::*, sequencer_types::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct InitializeClusterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
    cluster_id: ClusterId,
}

impl std::fmt::Display for InitializeClusterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitializeCluster {
    signature: Signature,
    message: InitializeClusterMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitializeClusterResponse {
    pub success: bool,
}

impl InitializeCluster {
    pub const METHOD_NAME: &'static str = "initialize_cluster";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<Publisher>,
    ) -> Result<InitializeClusterResponse, RpcError> {
        let parameter = parameter.parse::<InitializeCluster>()?;

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        // Todo: tmp
        let platform = match parameter.message.chain_type {
            ChainType::Ethereum => PlatForm::Ethereum,
            _ => PlatForm::Local,
        };

        match parameter.message.sequencing_function_type {
            SequencingFunctionType::Liveness => {
                match LivenessClusterModel::get(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        let cluster_model = LivenessClusterModel::new(
                            platform.clone(),
                            parameter.message.service_type.clone(),
                            parameter.message.cluster_id.clone(),
                        );

                        cluster_model.put()?;
                    }
                }
            }
            SequencingFunctionType::Validation => {
                match ValidationClusterModel::get(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        let cluster_model = ValidationClusterModel::new(
                            platform.clone(),
                            parameter.message.service_type.clone(),
                            parameter.message.cluster_id.clone(),
                        );

                        cluster_model.put()?;
                    }
                }
            }
        }

        match ClusterIdListModel::get_mut(
            &platform,
            &parameter.message.sequencing_function_type,
            &parameter.message.service_type,
        ) {
            Ok(mut cluster_id_list) => {
                cluster_id_list.add_cluster_id(parameter.message.cluster_id);
                cluster_id_list.update()?;
            }
            Err(err) => {
                if err.is_none_type() {
                    let mut cluster_id_list_model = ClusterIdListModel::default();
                    cluster_id_list_model.add_cluster_id(parameter.message.cluster_id);
                    cluster_id_list_model.put(
                        &platform,
                        &parameter.message.sequencing_function_type,
                        &parameter.message.service_type,
                    )?;
                } else {
                    return Err(err.into());
                }
            }
        };

        Ok(InitializeClusterResponse { success: true })
    }
}
