use std::{fs, str};

use anyhow::Result;
use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

use crate::options::Options;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub payer_path: String,
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
        if let Some(payer_path) = &options.payer_path {
            self.rpc_url = payer_path.to_owned();
        }
        self
    }
}
