use crate::lang::node::version::{NodeVersion, NodeVersions};

fn fetch_version() -> Result<Vec<NodeVersion>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://nodejs.org/download/release/index.json")?.text()?;
    let versions: Vec<NodeVersion> = serde_json::from_str(&body)?;
    Ok(versions)
}

pub fn remote_execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut versions = NodeVersions::new(fetch_version()?);
    versions.parse_versions();

    if let Some(current) = versions.latest_current() {
        println!("{} - {} (Current)", current.version, current.date);
    }

    let mut lts_versions: Vec<_> = versions.latest_lts().values().cloned().collect();
    lts_versions.sort_by(|a, b| b.parsed_version.cmp(&a.parsed_version));

    for v in &lts_versions {
        println!(
            "{} - {} LTS ({})",
            v.version,
            v.date,
            v.lts.as_ref().unwrap()
        );
    }

    Ok(())
}

pub fn all_remote_execute() -> Result<(), Box<dyn std::error::Error>> {
    let version = fetch_version()?;
    for v in &version {
        println!(
            "{} - {} {}",
            v.version,
            v.date,
            v.lts.as_deref().unwrap_or("")
        );
    }

    Ok(())
}
