use serde::{Deserialize, Serialize};

pub type SequencerIndex = usize;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
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
