use radius_sequencer_sdk::liveness::types::Address;
use serde::{Deserialize, Serialize};

use super::prelude::IpAddress;
use crate::{
    error::Error, models::prelude::ClusterInfoModel, sequencer_types::prelude::SequencingInfoKey,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ClusterId(String);

impl std::fmt::Display for ClusterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ClusterId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ClusterIdList(Vec<ClusterId>);

impl AsRef<Vec<ClusterId>> for ClusterIdList {
    fn as_ref(&self) -> &Vec<ClusterId> {
        &self.0
    }
}

impl AsMut<Vec<ClusterId>> for ClusterIdList {
    fn as_mut(&mut self) -> &mut Vec<ClusterId> {
        &mut self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ClusterInfo {
    cluster_id: ClusterId,
    sequencing_info_key: SequencingInfoKey,
    sequencer_rpc_url_list: Vec<(Address, Option<IpAddress>)>,
}

impl From<ClusterInfoModel> for ClusterInfo {
    fn from(cluster_info_model: ClusterInfoModel) -> Self {
        Self {
            cluster_id: cluster_info_model.cluster_id().clone(),
            sequencing_info_key: cluster_info_model.sequencing_info_key(),
            sequencer_rpc_url_list: cluster_info_model.sequencer_rpc_url_list().clone(),
        }
    }
}

impl ClusterInfo {
    pub fn new(
        cluster_id: ClusterId,
        sequencing_info_key: SequencingInfoKey,
        sequencer_rpc_url_list: Vec<(Address, Option<IpAddress>)>,
    ) -> Self {
        Self {
            cluster_id,
            sequencing_info_key,
            sequencer_rpc_url_list,
        }
    }

    pub fn cluster_id(&self) -> &ClusterId {
        &self.cluster_id
    }

    pub fn sequencing_info_key(&self) -> SequencingInfoKey {
        self.sequencing_info_key
    }

    pub fn sequencer_rpc_url_list(&self) -> &Vec<(Address, Option<IpAddress>)> {
        &self.sequencer_rpc_url_list
    }

    pub fn add_sequencer(
        &mut self,
        sequencer: Address,
        rpc_url: Option<IpAddress>,
    ) -> Result<(), Error> {
        if self.is_exist_sequencer(&sequencer) {
            return Err(Error::AlreadyRegisteredSequencer);
        }

        self.sequencer_rpc_url_list.push((sequencer, rpc_url));
        Ok(())
    }

    pub fn remove_sequencer(&mut self, sequencer: &Address) -> Result<(), Error> {
        let index = self
            .sequencer_rpc_url_list
            .iter()
            .position(|(address, _)| address == sequencer)
            .ok_or(Error::SequencerNotRegistered)?;

        self.sequencer_rpc_url_list.remove(index);
        Ok(())
    }

    pub fn is_exist_sequencer(&self, sequencer: &Address) -> bool {
        self.sequencer_rpc_url_list
            .iter()
            .any(|(address, _)| address == sequencer)
    }
}
