//! Operating System specific functions

use std::env::consts::{ARCH, OS};

use serde::{Deserialize, Serialize};
use strum::EnumString;
use thiserror::Error;

// Note: Non-camel-case is allowed because using the wrong case for proper
//  names bothers the original developer

/// The type of Operating System on which the application is hosted
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, EnumString)]
pub enum OperatingSystem {
    Linux,
    Android,
    BSD,
    MacOS,
    #[allow(non_camel_case_types)]
    iOs,
    Windows,
    WebAssembly,
}

impl OperatingSystem {
    /// Identify the current OS
    pub fn identify() -> Result<Self, OSError> {
        Ok(match OS {
            "linux" => OperatingSystem::Linux,
            "windows" => OperatingSystem::Windows,
            "macos" => OperatingSystem::MacOS,
            "android" => OperatingSystem::Android,
            "wasi" => OperatingSystem::WebAssembly,
            "openbsd" | "freebsd" | "netbsd" => OperatingSystem::BSD,
            _ => return Err(OSError::UnsupportedOS(OS.to_string())),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, EnumString)]
#[allow(non_camel_case_types)]
pub enum Architecture {
    x86_64,
    arm,
    aarch64,
}

impl Architecture {
    pub fn identify() -> Result<Self, OSError> {
        Ok(match ARCH {
            "x86_64" => Architecture::x86_64,
            "arm" => Architecture::arm,
            "aarch64" => Architecture::aarch64,
            _ => return Err(OSError::UnsupportedArchitecture(ARCH.to_string())),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum OSError {
    #[error("Unsupported operating system {0}")]
    UnsupportedOS(String),
    #[error("Unsupported architecture {0}")]
    UnsupportedArchitecture(String),
}
