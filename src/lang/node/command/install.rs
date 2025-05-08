use std::fs::{self};

use crate::{
    core::{
        archive::{ArchiveError, extract_archive},
        downloads::{DownloadError, download_file},
        fs::{FsError, ensure_node_downloads_dir, ensure_node_versions_dir},
    },
    info,
    platform::{detect_arch, detect_plateform},
    success, verbose,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("Error filesystem handling: {0}")]
    Fs(#[from] FsError),

    #[error("Error occured while downloading: {0}")]
    Download(#[from] DownloadError),

    #[error("Archive error: {0}")]
    Archive(#[from] ArchiveError),

    #[error("System error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn execute(version: &str) -> Result<(), InstallError> {
    let destination_path = ensure_node_versions_dir()?;
    let version_installation_folder = destination_path.join(version);
    if version_installation_folder.exists() {
        verbose!(
            "Node.js {} is already installed at: {}",
            version,
            destination_path.display()
        );
        info!("Node.js {} is already installed", version);

        return Ok(());
    }
    info!("Installating Node.js {} ...", version);

    let plateform = detect_plateform();
    let arch = detect_arch(); // x64, arm64

    let url = build_download_node_url(version, &plateform, &arch);
    // TODO: refactor this to have one archive name builder
    let archive_path = ensure_node_downloads_dir()?.join(format!(
        "node-{v}-{plateform}-{arch}.{ext}",
        v = version,
        plateform = plateform,
        arch = arch,
        ext = if plateform == "win" { "7z" } else { "tar.gz" }
    ));

    if archive_path.exists() {
        info!("Archive already exist skip downloading");
    } else {
        verbose!("Downloading Node.js from {}", url);
        download_file(&url, &archive_path)?;
        success!("Downloaded Node.js {}", version);
    }

    extract_archive(&archive_path, &destination_path)?;

    let extraction_folder = destination_path.join(format!(
        "node-{v}-{plateform}-{arch}",
        v = version,
        plateform = plateform,
        arch = arch
    ));

    verbose!("Extraction folder {}", extraction_folder.display());
    fs::rename(&extraction_folder, version_installation_folder)?;

    verbose!(
        "Node.js {} is installed at: {}",
        version,
        destination_path.display()
    );

    success!("Installed Node.js {}", version);

    Ok(())
}

fn build_download_node_url(version: &str, plateform: &str, arch: &str) -> String {
    let base_url = "https://nodejs.org/download/release";

    format!(
        "{base_url}/{v}/node-{v}-{plateform}-{arch}.{ext}",
        base_url = base_url,
        v = version,
        plateform = plateform,
        arch = arch,
        ext = if plateform == "win" { "7z" } else { "tar.gz" }
    )
}
