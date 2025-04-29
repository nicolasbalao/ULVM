use std::{
    env, fs,
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

        Self::parse_config_file(path)
    }

    pub fn load() -> Result<Self, UlvmConfigError> {
        Self::load_current_path().or_else(|_| Self::load_base())
    }

    pub fn load_base() -> Result<Self, UlvmConfigError> {
        Self::parse_config_file(Self::base_config_file_path()?)
    }

    pub fn load_current_path() -> Result<Self, UlvmConfigError> {
        let current_dir = env::current_dir()?;
        let current_dir_config_path = current_dir.join("ulvm.toml");

        Self::parse_config_file(current_dir_config_path)
    }

    fn parse_config_file(config_path: PathBuf) -> Result<Self, UlvmConfigError> {
        let config_content = fs::read_to_string(&config_path)?;

        let config: UlvmConfig = toml::from_str(&config_content).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Failed to parse config file")
        })?;

        Ok(config)
    }

    fn base_config_file_path() -> Result<PathBuf, FsError> {
        Ok(ulvm_fs::ensure_ulvm_home_dir()?.join("ulvm.toml"))
    }
}
