use std::io;
use thiserror::Error;
use toml::ser::Error as TomlError;

use crate::core::fs::FsError;

#[derive(Error, Debug)]
pub enum UlvmConfigError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] TomlError),

    #[error("Error filesystem handling: {0}")]
    UlvmFs(#[from] FsError),
}
