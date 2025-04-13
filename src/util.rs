use std::{fs, io, path::Path, time::Duration};

use reqwest::Client;

use crate::error::Error;

pub async fn health_check(tx_orderer_rpc_url: impl AsRef<str>) -> Result<(), Error> {
    let health_check_url = format!("{}/health", tx_orderer_rpc_url.as_ref());

    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(Error::InvalidURL)?;

    client
        .get(health_check_url)
        .send()
        .await
        .map_err(Error::HealthCheck)?;

    Ok(())
}

pub fn clear_dir<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    if path.as_ref().exists() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}
