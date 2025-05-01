use std::{io, path::Path};

use sevenz_rust::Error as SevenZError;
use thiserror::Error;

use crate::platform;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Gzip decompression error: {0}")]
    GzDecode(#[from] flate2::DecompressError),

    #[error("7z decompress error: {0}")]
    SevenZ(#[from] SevenZError),

    #[error("Invalid archive entry path")]
    InvalidEntryPath,
}

#[cfg(unix)]
pub fn extract_archive(source_path: &PathBuf, destination_path: &Path) -> Result<(), ArchiveError> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::path::PathBuf;
    use tar::Archive;

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

#[cfg(windows)]
pub fn extract_archive(archive: &Path, destination: &Path) -> Result<(), ArchiveError> {
    use sevenz_rust::decompress_file;
    println!(
        "Decompress archive file {} to {}",
        archive.display(),
        destination.display()
    );
    decompress_file(archive, destination)?;
    Ok(())
}

pub fn build_archive_name(version: &str) -> String {
    let plateform = platform::detect_plateform();
    let arch = platform::detect_arch();

    let ext = if cfg!(target_family = "windows") {
        "7z"
    } else {
        "tar.gz"
    };

    format!(
        "node-{v}-{p}-{a}.{ext}",
        v = &version,
        p = plateform,
        a = arch,
        ext = ext
    )
}
