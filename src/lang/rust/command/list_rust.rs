use serde::Deserialize;

use crate::{lang::rust::fs_rust::find_rust_installed_versions, ui::display_rust_version, warn};

#[derive(Deserialize, Debug)]
pub struct RustVersion {
    pub name: String,
}

impl RustVersion {
    pub fn is_installed(&self) -> bool {
        let installed_version: Vec<String> = find_rust_installed_versions().unwrap();
        installed_version.contains(&self.name.to_string())
    }
}

pub fn execute() {
    warn!("Not implemented yet");

    let http_client = reqwest::blocking::Client::new();
    let json = http_client
        .get(" https://api.github.com/repos/rust-lang/rust/tags")
        .header("User-Agent", "ulvm")
        .send()
        .expect("Failed request")
        .text()
        .unwrap();

    let versions: Vec<RustVersion> =
        serde_json::from_str(&json).expect("Failed to parse json to rust version");

    let valid_versions: Vec<&RustVersion> = versions
        .iter()
        .filter(|v| is_valid_rust_version(&v.name))
        .collect();
    display_rust_version(valid_versions);
}

fn is_valid_rust_version(tag_name: &str) -> bool {
    !tag_name.starts_with("release")
}
