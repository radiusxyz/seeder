use serde::{Deserialize, Serialize};

use crate::{error::Error, types::prelude::Platform};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: String) -> Self {
        if !address.starts_with("0x") {
            Self(format!("0x{}", address.to_lowercase()))
        } else {
            Self(address.to_lowercase())
        }
    }

    pub fn get_platform_address(
        &self,
        platform: Platform,
    ) -> Result<radius_sequencer_sdk::signature::Address, Error> {
        Ok(radius_sequencer_sdk::signature::Address::from_str(
            platform.into(),
            self.0.as_str(),
        )?)
    }
}

pub fn fmt_hex_string(f: &mut std::fmt::Formatter, data: &[u8]) -> std::fmt::Result {
    f.write_str("0x")?;
    data.iter()
        .try_for_each(|byte| f.write_fmt(format_args!("{:02x}", byte)))
}

pub fn to_sdk_platform(platform: Platform) -> radius_sequencer_sdk::signature::Platform {
    match platform {
        Platform::Ethereum => radius_sequencer_sdk::signature::Platform::Ethereum,
        Platform::Local => radius_sequencer_sdk::signature::Platform::Ethereum,
    }
}
