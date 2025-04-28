use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::core::fs::{self as ulvm_fs, FsError};

use super::errors::UlvmConfigError;

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeConfig {
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UlvmConfig {
    pub node: Option<NodeConfig>,
}

impl UlvmConfig {
    pub fn save(&self) -> Result<(), UlvmConfigError> {
        let toml_str = toml::to_string(&self)?;
        let base_config_path = Self::base_config_file_path()?;

        let mut config_file = fs::File::create(&base_config_path)?;
        config_file.write_all(toml_str.as_bytes())?;

        Ok(())
    }

    pub fn load_base_or_create() -> Result<Self, UlvmConfigError> {
        let path = Self::base_config_file_path()?;

        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let toml_config = fs::read_to_string(&path)?;
        let config: UlvmConfig = toml::from_str(&toml_config).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Failed to parse config file")
        })?;

        Ok(config)
    }

    fn base_config_file_path() -> Result<PathBuf, FsError> {
        Ok(ulvm_fs::ensure_ulvm_home_dir()?.join("ulvm.toml"))
    }
}
