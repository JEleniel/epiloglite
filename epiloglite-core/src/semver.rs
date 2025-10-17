use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Semantic Versioning struct for EpilogLite.
///
/// Represents a version in the form `vMAJOR.MINOR.REVISION[-tag]`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SemVer {
    /// Major version number (breaking changes)
    pub major: u8,
    /// Minor version number (feature additions)
    pub minor: u8,
    /// Revision version number (bug fixes)
    pub revision: u8,
    /// Optional tag (e.g., alpha, beta, rc)
    pub tag: String,
}

impl TryFrom<&str> for SemVer {
    type Error = SemVerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let value = if value.to_lowercase().starts_with('v') {
            &value[1..]
        } else {
            value
        };

        // Split off tag if present
        let (core, tag) = match value.split_once('-') {
            Some((core, tag)) => (core, tag),
            None => (value, ""),
        };

        let mut parts = core.splitn(3, '.');
        let major_str = parts.next().ok_or(SemVerError::InvalidFormat)?;
        let minor_str = parts.next().ok_or(SemVerError::InvalidFormat)?;
        let revision_str = parts.next().ok_or(SemVerError::InvalidFormat)?;

        let major: u8 = major_str
            .parse()
            .map_err(|_| SemVerError::InvalidMajorVersion(major_str.to_string()))?;
        let minor: u8 = minor_str
            .parse()
            .map_err(|_| SemVerError::InvalidMinorVersion(minor_str.to_string()))?;
        let revision: u8 = revision_str
            .parse()
            .map_err(|_| SemVerError::InvalidRevision(revision_str.to_string()))?;

        Ok(Self {
            major,
            minor,
            revision,
            tag: tag.to_string(),
        })
    }
}

impl Into<String> for SemVer {
    fn into(self) -> String {
        format!(
            "v{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.revision,
            if !self.tag.is_empty() { "-" } else { "" },
            self.tag
        )
    }
}

impl TryFrom<u32> for SemVer {
    type Error = SemVerError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let revision: u8 = match (value % 1000).try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(SemVerError::InvalidU32(value));
            }
        };
        let r1 = (value - revision as u32) / 1000;
        let minor: u8 = match (r1 % 1000).try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(SemVerError::InvalidU32(value));
            }
        };
        let major: u8 = match ((r1 - minor as u32) / 1000).try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(SemVerError::InvalidU32(value));
            }
        };

        Ok(Self {
            major,
            minor,
            revision,
            tag: String::new(),
        })
    }
}

impl Into<u32> for SemVer {
    fn into(self) -> u32 {
        self.major as u32 * 1000000 + self.minor as u32 * 1000 + self.revision as u32
    }
}

/// Errors that can occur when parsing or converting semantic versions.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum SemVerError {
    /// The input string does not match the expected format.
    #[error("Invalid SemVer format; use #.#.#-tag or v#.#.#-tag")]
    InvalidFormat,
    /// The major version component is invalid.
    #[error("Invalid major version {0}")]
    InvalidMajorVersion(String),
    /// The minor version component is invalid.
    #[error("Invalid minor version {0}")]
    InvalidMinorVersion(String),
    /// The revision version component is invalid.
    #[error("Invalid revision {0}")]
    InvalidRevision(String),
    /// The u32 representation is invalid for conversion.
    #[error("Invalid u32 {0}")]
    InvalidU32(u32),
}
