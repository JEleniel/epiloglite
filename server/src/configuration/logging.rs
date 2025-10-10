use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Logging {
    pub root_level: log::Level,
    pub enable_std: bool,
    pub enable_syslog: bool,
    pub enable_file: bool,
    pub log_path: Option<PathBuf>,
    pub targets: LogTarget,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogTarget {
    pub name: String,
    pub level: log::Level,
}

impl Logging {
    pub fn init(&self) {}
}
