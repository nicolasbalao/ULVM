use crate::{
    lang::node::version::{NodeVersion, NodeVersions},
    ui::display_versions,
};

fn fetch_version() -> Result<Vec<NodeVersion>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://nodejs.org/download/release/index.json")?.text()?;
    let versions: Vec<NodeVersion> = serde_json::from_str(&body)?;
    Ok(versions)
}

pub fn remote_execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut versions = NodeVersions::new(fetch_version()?);
    versions.parse_versions();
    versions.assign_status();
    versions.process_versions()?;

    let mut lts_versions: Vec<&NodeVersion> = versions.latest_lts().values().cloned().collect();
    lts_versions.sort_by(|a, b| b.parsed_version.cmp(&a.parsed_version));

    lts_versions.insert(0, versions.latest_current().unwrap());

    display_versions(lts_versions);

    Ok(())
}

pub fn all_remote_execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut node_versions = NodeVersions::new(fetch_version()?);
    node_versions.parse_versions();
    node_versions.assign_status();
    node_versions.process_versions()?;

    let versions = node_versions.versions.iter().collect();
    display_versions(versions);

    Ok(())
}
