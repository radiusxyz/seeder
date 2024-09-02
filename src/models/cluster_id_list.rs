use crate::{
    models::prelude::*,
    sequencer_types::prelude::{
        ClusterId, ClusterIdList, Platform, SequencingFunctionType, ServiceProvider,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ClusterIdListModel {
    cluster_id_list: ClusterIdList,
}

impl ClusterIdListModel {
    pub fn new(cluster_id_list: ClusterIdList) -> Self {
        Self { cluster_id_list }
    }

    pub fn cluster_id_list(self) -> ClusterIdList {
        self.cluster_id_list
    }

    pub fn add_cluster_id(&mut self, cluster_id: ClusterId) {
        if !self.is_exist_cluster_id(&cluster_id) {
            self.cluster_id_list.as_mut().push(cluster_id);
        }
    }

    pub fn is_exist_cluster_id(&self, cluster_id: &ClusterId) -> bool {
        self.cluster_id_list.as_ref().contains(cluster_id)
    }
}

impl ClusterIdListModel {
    pub const ID: &'static str = stringify!(ClusterIdListModel);

    pub fn get(
        platform: Platform,
        sequencing_function_type: SequencingFunctionType,
        service_provider: ServiceProvider,
    ) -> Result<Self, DbError> {
        let key = (
            Self::ID,
            platform,
            sequencing_function_type,
            service_provider,
        );
        database()?.get(&key)
    }

    pub fn get_mut(
        platform: Platform,
        sequencing_function_type: SequencingFunctionType,
        service_provider: ServiceProvider,
    ) -> Result<Lock<'static, Self>, DbError> {
        let key = (
            Self::ID,
            platform,
            sequencing_function_type,
            service_provider,
        );
        database()?.get_mut(&key)
    }

    pub fn put(
        &self,
        platform: Platform,
        sequencing_function_type: SequencingFunctionType,
        service_provider: ServiceProvider,
    ) -> Result<(), DbError> {
        let key = (
            Self::ID,
            platform,
            sequencing_function_type,
            service_provider,
        );
        database()?.put(&key, self)
    }
}
