#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Semver {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Semver {
    pub fn parse(version: &str) -> Option<Self> {
        let version = version.trim_start_matches("v");
        let part: Vec<_> = version.split(".").collect();
        if part.len() != 3 {
            return None;
        }

        Some(Semver {
            major: part[0].parse().ok()?,
            minor: part[1].parse().ok()?,
            patch: part[2].parse().ok()?,
        })
    }
}
