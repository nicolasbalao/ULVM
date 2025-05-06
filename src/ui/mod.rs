use colored::Colorize;

/// Icônes standardisées
pub const ICON_SUCCESS: &str = "✅️";
pub const ICON_ERROR: &str = "❌";
pub const ICON_WARN: &str = "⚠️";
pub const ICON_INFO: &str = "ℹ️";
pub const ICON_ACTIVE: &str = "➡️";

pub fn success(msg: &str) {
    println!("{} {}", ICON_SUCCESS, msg.green());
}

pub fn error(msg: &str) {
    eprintln!("{} {}", ICON_ERROR, msg.red());
}

pub fn warn(msg: &str) {
    println!("{} {}", ICON_WARN, msg.yellow());
}

pub fn info(msg: &str) {
    println!("{}  {}", ICON_INFO, msg.cyan());
}
