use std::fs::{self};

use crate::{
    core::{
        archive::{ArchiveError, extract_archive},
        downloads::{DownloadError, download_file},
        fs::{FsError, ensure_node_downloads_dir, ensure_node_versions_dir},
    },
    platform::{detect_arch, detect_plateform},
    ui,
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
    ui::info(format!("Installation node.js {} ...", version).as_str());
    let destination_path = ensure_node_versions_dir()?;
    let version_installation_folder = destination_path.join(version);
    if version_installation_folder.exists() {
        ui::info(
            format!(
                "Nodejs {} is already installed at: {}",
                version,
                destination_path.display()
            )
            .as_str(),
        );

        return Ok(());
    }

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
        ui::info("Archive already exist skip downloading");
    } else {
        ui::info(format!("Downloading Node.js from {}", url).as_str());
        download_file(&url, &archive_path)?;
    }

    extract_archive(&archive_path, &destination_path)?;

    let extraction_folder = destination_path.join(format!(
        "node-{v}-{plateform}-{arch}",
        v = version,
        plateform = plateform,
        arch = arch
    ));

    ui::info(format!("Extraction folder {}", extraction_folder.display()).as_str());

    fs::rename(extraction_folder, version_installation_folder)?;

    ui::success(
        format!(
            "Nodejs {} is installed at: {}",
            version,
            destination_path.display()
        )
        .as_str(),
    );

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
