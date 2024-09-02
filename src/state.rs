use std::{collections::HashMap, sync::Arc};

use radius_sequencer_sdk::{
    context::{Context, SharedContext},
    liveness::{publisher::Publisher, types::Address},
};

use crate::{
    cli::Config,
    error::Error,
    sequencer_types::prelude::{
        ClusterId, ClusterInfo, IpAddress, SequencingInfoKey, SequencingInfoPayload,
    },
};

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    sequencing_infos: SharedContext<HashMap<SequencingInfoKey, SequencingInfoPayload>>,
    cluster_infos: SharedContext<HashMap<ClusterId, ClusterInfo>>,
    publishers: SharedContext<HashMap<SequencingInfoKey, Arc<Publisher>>>,
}

unsafe impl Send for AppState {}

unsafe impl Sync for AppState {}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AppState {
    pub fn new(
        config: Config,
        sequencing_infos: HashMap<SequencingInfoKey, SequencingInfoPayload>,
        cluster_info: HashMap<ClusterId, ClusterInfo>,
        publisher: HashMap<SequencingInfoKey, Arc<Publisher>>,
    ) -> Self {
        let inner = AppStateInner {
            config,
            sequencing_infos: SharedContext::from(sequencing_infos),
            cluster_infos: SharedContext::from(cluster_info),
            publishers: SharedContext::from(publisher),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    pub fn sequencing_infos(&self) -> Context<HashMap<SequencingInfoKey, SequencingInfoPayload>> {
        self.inner.sequencing_infos.load()
    }

    pub fn cluster_infos(&self) -> Context<HashMap<ClusterId, ClusterInfo>> {
        self.inner.cluster_infos.load()
    }

    pub fn publishers(&self) -> Context<HashMap<SequencingInfoKey, Arc<Publisher>>> {
        self.inner.publishers.load()
    }

    pub fn get_sequencing_info(
        &self,
        sequencing_info_key: SequencingInfoKey,
    ) -> Result<SequencingInfoPayload, Error> {
        self.sequencing_infos()
            .as_ref()
            .get(&sequencing_info_key)
            .cloned()
            .ok_or(Error::FailedToGetSequencingInfo)
    }

    pub fn get_cluster_info(&self, cluster_id: &ClusterId) -> Result<ClusterInfo, Error> {
        self.cluster_infos()
            .as_ref()
            .get(cluster_id)
            .cloned()
            .ok_or(Error::ClusterNotRegistered)
    }

    pub fn get_publisher(
        &self,
        sequencing_info_key: SequencingInfoKey,
    ) -> Result<Arc<Publisher>, Error> {
        self.publishers()
            .as_ref()
            .get(&sequencing_info_key)
            .cloned()
            .ok_or(Error::FailedToGetPublisher)
    }

    pub fn add_sequencing_info(
        &self,
        sequencing_info_key: SequencingInfoKey,
        sequencing_info_payload: SequencingInfoPayload,
    ) {
        let mut sequencing_infos = self.sequencing_infos().as_ref().clone();
        sequencing_infos.insert(sequencing_info_key, sequencing_info_payload);

        self.inner.sequencing_infos.store(sequencing_infos);
    }

    pub fn add_cluster_info(&self, cluster_id: ClusterId, cluster_info: ClusterInfo) {
        let mut cluster_infos = self.cluster_infos().as_ref().clone();
        cluster_infos.insert(cluster_id, cluster_info);

        self.inner.cluster_infos.store(cluster_infos);
    }

    pub fn add_sequencer_to_cluster(
        &self,
        cluster_id: &ClusterId,
        sequencer: Address,
        rpc_url: Option<IpAddress>,
    ) -> Result<(), Error> {
        let mut cluster_info = self.get_cluster_info(cluster_id)?;
        cluster_info.add_sequencer(sequencer, rpc_url)?;

        self.add_cluster_info(cluster_id.clone(), cluster_info);

        Ok(())
    }

    pub fn add_publisher(&self, sequencing_info_key: SequencingInfoKey, publisher: Arc<Publisher>) {
        let mut publishers = self.publishers().as_ref().clone();
        publishers.insert(sequencing_info_key, publisher);

        self.inner.publishers.store(publishers);
    }
}
