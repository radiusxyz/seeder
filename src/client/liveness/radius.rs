use std::sync::Arc;

use radius_sdk::{liveness::radius::publisher::Publisher, signature::PrivateKeySigner};

use crate::{error::Error, state::AppState, types::*};

pub struct LivenessClient {
    inner: Arc<LivenessClientInner>,
}

struct LivenessClientInner {
    platform: Platform,
    liveness_service_provider: LivenessServiceProvider,
    publisher: Publisher,
}

impl Clone for LivenessClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl LivenessClient {
    pub fn platform(&self) -> Platform {
        self.inner.platform
    }

    pub fn liveness_service_provider(&self) -> LivenessServiceProvider {
        self.inner.liveness_service_provider
    }

    pub fn publisher(&self) -> &Publisher {
        &self.inner.publisher
    }

    pub fn new(
        platform: Platform,
        liveness_service_provider: LivenessServiceProvider,
        liveness_info: LivenessRadius,
        signing_key: impl AsRef<str>,
    ) -> Result<Self, Error> {
        let publisher = Publisher::new(
            liveness_info.liveness_rpc_url,
            signing_key,
            &liveness_info.contract_address,
        )
        .map_err(|error| Error::LivenessClient(error.into()))?;

        let inner = LivenessClientInner {
            platform,
            liveness_service_provider,
            publisher,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn initialize(
        context: AppState,
        platform: Platform,
        liveness_service_provider: LivenessServiceProvider,
        liveness_info: LivenessRadius,
    ) {
        tokio::spawn({
            let context = context.clone();
            let liveness_info = liveness_info.clone();

            async move {
                let signing_key = &context.config().signing_key;
                let signer = PrivateKeySigner::from_str(platform.into(), signing_key).unwrap();
                context.add_signer(platform, signer).await.unwrap();

                let liveness_client = Self::new(
                    platform,
                    liveness_service_provider,
                    liveness_info,
                    signing_key,
                )
                .unwrap();

                context
                    .add_liveness_client(
                        platform,
                        liveness_service_provider,
                        liveness_client.clone(),
                    )
                    .await
                    .unwrap();

                tracing::info!(
                    "Initializing the liveness event listener for {:?}, {:?}..",
                    platform,
                    liveness_service_provider
                );
            }
        });
    }
}
