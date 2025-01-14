use std::{any::Any, sync::Arc};

use radius_sdk::{
    kvstore::{CachedKvStore, CachedKvStoreError},
    signature::PrivateKeySigner,
};

use crate::types::{Config, Platform, ServiceProvider};

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    liveness_clients: CachedKvStore,
    signers: CachedKvStore,
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
    pub fn new(config: Config, liveness_clients: CachedKvStore, signers: CachedKvStore) -> Self {
        let inner = AppStateInner {
            config,
            liveness_clients,
            signers,
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }
}

/// Liveness client functions
impl AppState {
    pub async fn add_liveness_client<T>(
        &self,
        platform: Platform,
        service_provider: ServiceProvider,
        liveness_client: T,
    ) -> Result<(), CachedKvStoreError>
    where
        T: Clone + Any + Send + 'static,
    {
        let key = &(platform, service_provider);

        self.inner.liveness_clients.put(key, liveness_client).await
    }

    pub async fn get_liveness_client<T>(
        &self,
        platform: Platform,
        service_provider: ServiceProvider,
    ) -> Result<T, CachedKvStoreError>
    where
        T: Clone + Any + Send + 'static,
    {
        let key = &(platform, service_provider);

        self.inner.liveness_clients.get(key).await
    }
}

/// Signer functions
impl AppState {
    pub async fn add_signer(
        &self,
        platform: Platform,
        signer: PrivateKeySigner,
    ) -> Result<(), CachedKvStoreError> {
        let key = &(platform);

        self.inner.signers.put(key, signer).await
    }

    pub async fn get_signer(
        &self,
        platform: Platform,
    ) -> Result<PrivateKeySigner, CachedKvStoreError> {
        let key = &(platform);

        self.inner.signers.get(key).await
    }
}
