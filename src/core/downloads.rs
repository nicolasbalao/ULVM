use std::{fs::File, io::Write, path::Path};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Download failed, Http status : {0}")]
    HttpStatus(reqwest::StatusCode),
}

pub fn download_file(url: &str, destination: &Path) -> Result<(), DownloadError> {
    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        return Err(DownloadError::HttpStatus(response.status()));
    }

    let mut dest = File::create(destination)?;
    let content = response.bytes()?;
    dest.write_all(&content)?;
    Ok(())
}
