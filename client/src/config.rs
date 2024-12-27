use std::{fs, str};

use anyhow::Result;
use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub rpc_url: String,
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
}
