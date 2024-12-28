use std::{
    fs,
    str::{self, FromStr},
};

use anyhow::Result;
use config::{ConfigError, File, FileFormat};
use serde::{Deserialize, Deserializer};
use solana_sdk::pubkey::Pubkey;

use crate::options::Options;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub ws_url: String,
    pub payer_path: String,
    #[serde(deserialize_with = "deserialize_pubkey_from_string")]
    pub raydium_v3_program: Pubkey,
}

impl Config {
    pub fn from_path(path: &str) -> Result<Self> {
        let toml = fs::read(path)?;
        let toml = str::from_utf8(toml.as_ref())?;

        let config_builder = config::Config::builder()
            .add_source(File::from_str(toml, FileFormat::Toml))
            .build()?;
        let result: Result<Self, ConfigError> = config_builder.try_deserialize();
        match result {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    pub fn merge_with_options(mut self, options: &Options) -> Self {
        if let Some(rpc_url) = &options.rpc_url {
            self.rpc_url = rpc_url.to_owned();
        }
        if let Some(ws_url) = &options.ws_url {
            self.ws_url = ws_url.to_owned();
        }
        if let Some(payer_path) = &options.payer_path {
            self.payer_path = payer_path.to_owned();
        }
        self
    }
}

fn deserialize_pubkey_from_string<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}
