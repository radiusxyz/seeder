use radius_sequencer_sdk::liveness::types::Address;
use serde::{Deserialize, Serialize};

use crate::{error::Error, models::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterInfoModel {
    cluster_id: ClusterId,
    sequencing_info_key: SequencingInfoKey,
    sequencer_rpc_url_list: Vec<(Address, Option<IpAddress>)>,
}

impl ClusterInfoModel {
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
        if !self.is_contain_sequencer(&sequencer) {
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

    pub fn is_contain_sequencer(&self, sequencer: &Address) -> bool {
        self.sequencer_rpc_url_list
            .iter()
            .any(|(address, _)| address == sequencer)
    }
}

impl ClusterInfoModel {
    pub const ID: &'static str = stringify!(ClusterInfoModel);

    pub fn get(cluster_id: &ClusterId) -> Result<Self, DbError> {
        let key = (Self::ID, cluster_id);
        database()?.get(&key)
    }

    pub fn get_mut(cluster_id: &ClusterId) -> Result<Lock<'static, Self>, DbError> {
        let key = (Self::ID, cluster_id);
        database()?.get_mut(&key)
    }

    pub fn put(&self) -> Result<(), DbError> {
        let key = (Self::ID, &self.cluster_id);
        database()?.put(&key, self)
    }
}
