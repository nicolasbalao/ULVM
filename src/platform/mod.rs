pub fn detect_arch() -> String {
    match std::env::consts::ARCH {
        "x86_64" => "x64".to_string(),
        "aarch64" => "arm64".to_string(),
        other => other.to_string(),
    }
}

pub fn detect_plateform() -> String {
    match std::env::consts::OS {
        "linux" => "linux".to_string(),
        "windows" => "win".to_string(),
        "macos" => "darwin".to_string(),
        other => other.to_string(),
    }
}
