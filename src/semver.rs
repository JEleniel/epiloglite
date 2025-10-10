use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SemVer {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub tag: String,
}

impl TryFrom<&str> for SemVer {
    type Error = SemVerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut offset = 0;

        if !value.starts_with(&['v', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0']) {
            return Err(SemVerError::InvalidFormat);
        }

        if value.to_lowercase().starts_with('v') {
            offset = 1;
        }

        let minor_start = match value.find('.') {
            Some(x) => x + 1,
            None => return Err(SemVerError::InvalidFormat),
        };
        let revision_start = match value[minor_start..].find('.') {
            Some(x) => x + 1,
            None => return Err(SemVerError::InvalidFormat),
        };
        let tag_start = match value[revision_start..].find('-') {
            Some(x) => x + 1,
            None => 0,
        };

        let major_str = String::from(&value[offset..minor_start - 1]);
        let minor_str = String::from(&value[minor_start..revision_start - 1]);
        let revision_str = String::from(if tag_start > 0 {
            &value[revision_start..tag_start - 1]
        } else {
            &value[revision_start..]
        });
        let tag_str = if tag_start == 0 {
            String::new()
        } else {
            String::from(&value[tag_start..])
        };

        let major: u8 = match major_str.parse() {
            Ok(v) => v,
            Err(_) => return Err(SemVerError::InvalidMajorVersion(major_str)),
        };
        let minor: u8 = match minor_str.parse() {
            Ok(v) => v,
            Err(_) => return Err(SemVerError::InvalidMinorVersion(minor_str)),
        };
        let revision: u8 = match revision_str.parse() {
            Ok(v) => v,
            Err(_) => return Err(SemVerError::InvalidRevision(revision_str)),
        };

        Ok(Self {
            major,
            minor,
            revision,
            tag: tag_str,
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

/// SemVer errors
#[derive(Error, Debug, PartialEq, Clone)]
pub enum SemVerError {
    #[error("Invalid SemVer format; use #.#.#-tag or v#.#.#-tag")]
    InvalidFormat,
    #[error("Invalid major version {0}")]
    InvalidMajorVersion(String),
    #[error("Invalid minor version {0}")]
    InvalidMinorVersion(String),
    #[error("Invalid revision {0}")]
    InvalidRevision(String),
    #[error("Invalid u32 {0}")]
    InvalidU32(u32),
}
