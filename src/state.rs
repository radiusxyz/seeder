use std::{any::Any, sync::Arc};

use radius_sdk::{
    kvstore::{CachedKvStore, CachedKvStoreError},
    signature::PrivateKeySigner,
};
use serde::Serialize;

use crate::types::{Config, Platform, ServiceProvider};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    liveness_clients: CachedKvStore,
    signers: CachedKvStore,
}

impl AppState {
    pub fn new(config: Config, liveness_clients: CachedKvStore, signers: CachedKvStore) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
                liveness_clients,
                signers,
            }),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    async fn put_to_store<K, V>(
        &self,
        store: &CachedKvStore,
        key: K,
        value: V,
    ) -> Result<(), CachedKvStoreError>
    where
        K: Send + Sync + 'static + std::fmt::Debug + Serialize,
        V: Clone + Any + Send + 'static,
    {
        store.put(&key, value).await
    }

    async fn get_from_store<K, V>(
        &self,
        store: &CachedKvStore,
        key: K,
    ) -> Result<V, CachedKvStoreError>
    where
        K: Send + Sync + 'static + std::fmt::Debug + Serialize,
        V: Clone + Any + Send + 'static,
    {
        store.get(&key).await
    }

    /// Liveness client functions
    pub async fn add_liveness_client<T>(
        &self,
        platform: Platform,
        service_provider: ServiceProvider,
        liveness_client: T,
    ) -> Result<(), CachedKvStoreError>
    where
        T: Clone + Any + Send + 'static,
    {
        self.put_to_store(
            &self.inner.liveness_clients,
            (platform, service_provider),
            liveness_client,
        )
        .await
    }

    pub async fn get_liveness_client<T>(
        &self,
        platform: Platform,
        service_provider: ServiceProvider,
    ) -> Result<T, CachedKvStoreError>
    where
        T: Clone + Any + Send + 'static,
    {
        self.get_from_store(&self.inner.liveness_clients, (platform, service_provider))
            .await
    }

    /// Signer functions
    pub async fn add_signer(
        &self,
        platform: Platform,
        signer: PrivateKeySigner,
    ) -> Result<(), CachedKvStoreError> {
        self.put_to_store(&self.inner.signers, platform, signer)
            .await
    }

    pub async fn get_signer(
        &self,
        platform: Platform,
    ) -> Result<PrivateKeySigner, CachedKvStoreError> {
        self.get_from_store(&self.inner.signers, platform).await
    }
}
