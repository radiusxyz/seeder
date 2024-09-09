use std::{collections::BTreeMap, sync::Arc};

use radius_sequencer_sdk::liveness_radius::publisher::Publisher;
use tokio::sync::Mutex;

use crate::error::Error;

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    publishers: Mutex<BTreeMap<String, Arc<Publisher>>>,
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
    pub fn new(publisher: BTreeMap<String, Arc<Publisher>>) -> Self {
        let inner = AppStateInner {
            publishers: Mutex::new(publisher),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn get_publisher(&self, sequencing_info_key: &str) -> Result<Arc<Publisher>, Error> {
        self.inner
            .publishers
            .lock()
            .await
            .get(sequencing_info_key)
            .cloned()
            .ok_or(Error::FailedToGetPublisher)
    }

    pub async fn add_publisher(&self, sequencing_info_key: String, publisher: Arc<Publisher>) {
        self.inner
            .publishers
            .lock()
            .await
            .insert(sequencing_info_key, publisher);
    }
}
