use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use flate2::read::GzDecoder;
use tar::Archive;

pub fn execute(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installation node.js {version} ...");

    let plateform = detect_plateform();
    let arch = detect_arch(); // x64, arm64

    let url = build_node_url(version, &plateform, &arch);

    println!("Dowloading Node.js from {url}");

    let source_path = download_archive(&url)?;
    println!("Source path : {:?}", source_path);

    let destination_path =
        PathBuf::new().join(format!("/home/nicolas/.ulvm/node/{v}", v = version));

    println!("create dir : {:?}", &destination_path);
    fs::create_dir(&destination_path)?;

    println!("Destination path: {:?}", &destination_path);
    extract_archive(&source_path, &destination_path)?;
    // tmp_dir.close()?;

    Ok(())
}

fn extract_archive<'a>(
    source_path: &PathBuf,
    destination_path: &'a PathBuf,
) -> Result<&'a PathBuf, Box<dyn std::error::Error>> {
    let archive_file = File::open(source_path)?;
    let mut archive = Archive::new(GzDecoder::new(archive_file));

    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
            let path = entry.path()?;

            let mut components = path.components();
            components.next();

            let new_path = components.as_path().to_path_buf();
            if new_path.as_os_str().is_empty() {
                return Ok(new_path);
            }

            let final_path = destination_path.join(new_path);

            entry.unpack(&final_path)?;
            Ok(final_path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(destination_path)
}

fn download_archive(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // let tmp_dir = Builder::new().prefix("node-download").tempdir()?;
    let tmp_dir = PathBuf::from("/tmp/node-version");

    let response = reqwest::blocking::get(url)?;

    let fname = response
        .url()
        .path_segments()
        .and_then(|mut segment| segment.next_back())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap();

    println!("File to download {fname}");
    let fname = tmp_dir.join(fname);

    println!("Will be located under: {:?}", fname);
    let mut dest = File::create(&fname)?;

    let content = response.bytes()?;
    dest.write_all(&content)?;
    Ok(fname)
}

fn build_node_url(version: &str, plateform: &str, arch: &str) -> String {
    // https://nodejs.org/download/release/v22.15.0/
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

fn detect_arch() -> String {
    std::env::consts::ARCH.to_string()
}

fn detect_plateform() -> String {
    std::env::consts::OS.to_string()
}
