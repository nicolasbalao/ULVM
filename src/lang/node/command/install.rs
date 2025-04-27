use std::fs::{self};

use crate::{
    core::{
        archive::{ArchiveError, extract_tar_gz},
        downloads::{DownloadError, download_file},
        fs::{FsError, ensure_node_downloads_dir, ensure_node_versions_dir},
    },
    platform::{detect_arch, detect_plateform},
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
    println!("Installation node.js {version} ...");
    let destination_path = ensure_node_versions_dir()?.join(version);
    if destination_path.exists() {
        println!(
            "Nodejs {} is already installed at: {}",
            version,
            destination_path.display()
        );
        return Ok(());
    }

    let plateform = detect_plateform();
    let arch = detect_arch(); // x64, arm64

    let url = build_download_node_url(version, &plateform, &arch);
    let archive_path = ensure_node_downloads_dir()?.join(format!(
        "node-{v}-{plateform}-{arch}.tar.gz",
        v = version,
        plateform = plateform,
        arch = arch
    ));

    if archive_path.exists() {
        println!("Archive already exist skip downloading")
    } else {
        println!("Dowloading Node.js from {url}");
        download_file(&url, &archive_path)?;
    }

    fs::create_dir(&destination_path)?;

    extract_tar_gz(&archive_path, &destination_path)?;

    println!(
        "Nodejs {} is installed at: {}",
        version,
        destination_path.display()
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
        arch = if arch == "x86_64" { "x64" } else { "arm64" },
        ext = if plateform == "win" { "zip" } else { "tar.gz" }
    )
}
