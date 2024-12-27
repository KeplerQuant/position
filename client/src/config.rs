use std::{fs, path::Path, str};

use anyhow::Result;
use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

/// Settings struct is used to load and store the settings from the configuration files.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub rpc_url: String,
}

impl Settings {
    pub fn new(filepath: &str) -> Result<Self> {
        let toml = fs::read(Path::new(&format!("config/{}", filepath)))?;
        let toml = str::from_utf8(toml.as_ref())?;

        let config_builder = Config::builder()
            .add_source(File::from_str(toml, FileFormat::Toml))
            .build()?;
        let result: Result<Self, ConfigError> = config_builder.try_deserialize();
        match result {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }
}
