use std::sync::Arc;

use prelude::*;
use radius_sequencer_sdk::signature::{ChainType, Signature};

use crate::{
    error::Error,
    models::prelude::*,
    rpc::{methods::serialize_to_bincode, prelude::*},
    sequencer_types::*,
    state::AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AddClusterMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_provider: ServiceProvider,
    cluster_id: ClusterId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddCluster {
    signature: Signature,
    message: AddClusterMessage,
}

impl AddCluster {
    pub const METHOD_NAME: &'static str = "add_cluster";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<AddCluster>()?;

        // // verify siganture
        // parameter.signature.verify_signature(
        //     serialize_to_bincode(&parameter.message)?.as_slice(),
        //     &parameter.message.address,
        //     parameter.message.chain_type,
        // )?;

        let sequencing_info_key = SequencingInfoKey::new(
            Platform::from(parameter.message.chain_type),
            parameter.message.sequencing_function_type,
            parameter.message.service_provider,
        );

        context.get_sequencing_info(sequencing_info_key)?;

        if context
            .get_cluster_info(&parameter.message.cluster_id)
            .is_ok()
        {
            return Err(Error::AlreadyRegisteredCluster.into());
        }

        let cluster_info_model = ClusterInfoModel::new(
            parameter.message.cluster_id.clone(),
            sequencing_info_key,
            Vec::new(),
        );
        cluster_info_model.put()?;

        let mut cluster_id_list_model = ClusterIdListModel::get_mut(
            sequencing_info_key.platform(),
            sequencing_info_key.sequencing_function_type(),
            sequencing_info_key.service_provider(),
        )?;
        cluster_id_list_model.add_cluster_id(parameter.message.cluster_id);
        cluster_id_list_model.update()?;

        context.add_cluster_info(
            cluster_info_model.cluster_id().clone(),
            cluster_info_model.into(),
        );

        Ok(())
    }
}
