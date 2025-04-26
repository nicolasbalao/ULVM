use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::core::semver::Semver;

#[derive(Deserialize, Debug)]
pub struct NodeVersion {
    pub version: String,
    pub date: String,
    #[serde(deserialize_with = "deserialize_lts")]
    pub lts: Option<String>,

    #[serde(skip)]
    pub parsed_version: Option<Semver>,
}

fn deserialize_lts<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    match value {
        serde_json::Value::Bool(false) => Ok(None),
        serde_json::Value::String(s) => Ok(Some(s)),
        _ => Ok(None),
    }
}

impl NodeVersion {
    // TODO: make error handling
    pub fn parse_version(&mut self) {
        self.parsed_version = Some(Semver::parse(&self.version).unwrap());
    }
}

pub struct NodeVersions {
    pub versions: Vec<NodeVersion>,
}

impl NodeVersions {
    pub fn new(versions: Vec<NodeVersion>) -> Self {
        Self { versions }
    }

    pub fn parse_versions(&mut self) {
        for v in &mut self.versions {
            v.parse_version();
        }
    }

    pub fn latest_current(&self) -> Option<&NodeVersion> {
        self.versions
            .iter()
            .filter(|v| v.lts.is_none())
            .max_by_key(|v| v.parsed_version.as_ref())
    }

    pub fn latest_lts(&self) -> HashMap<&str, &NodeVersion> {
        let mut map = HashMap::new();
        for v in self.versions.iter().filter(|v| v.lts.is_some()) {
            let lts_name = v.lts.as_ref().unwrap().as_str();
            map.entry(lts_name)
                .and_modify(|existing: &mut &NodeVersion| {
                    if v.parsed_version > existing.parsed_version {
                        *existing = v;
                    }
                })
                .or_insert(v);
        }
        map
    }
}
