use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;
use thiserror::Error;

use crate::platform;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Gzip decompression error: {0}")]
    GzDecode(#[from] flate2::DecompressError),

    #[error("Invalid archive entry path")]
    InvalidEntryPath,
}
pub fn extract_tar_gz(source_path: &PathBuf, destination_path: &Path) -> Result<(), ArchiveError> {
    let archive_file = File::open(source_path)?;
    let decoder = GzDecoder::new(archive_file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path().map_err(|_| ArchiveError::InvalidEntryPath)?;

        let mut component = path.components();
        component.next();

        let new_path = destination_path.join(component.as_path());

        if new_path.as_os_str().is_empty() {
            continue;
        }
        entry.unpack(&new_path)?;
    }

    Ok(())
}

pub fn build_archive_name(version: &str) -> String {
    let plateform = platform::detect_plateform();
    let arch = platform::detect_arch();

    format!(
        "node-{v}-{p}-{a}.tar.gz",
        v = &version,
        p = plateform,
        a = arch
    )
}
