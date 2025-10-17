// Unit tests for src/semver.rs
// Covers SemVer struct, SemVerError enum, and their methods

use crate::semver::{SemVer, SemVerError};

#[test]
fn test_semver_try_from_str_valid() {
    let s = "1.2.3-alpha";
    let ver = SemVer::try_from(s).unwrap();
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.revision, 3);
    assert_eq!(ver.tag, "alpha");
}

#[test]
fn test_semver_try_from_str_invalid() {
    let s = "not.a.version";
    let err = SemVer::try_from(s);
    assert!(err.is_err());
}

#[test]
fn test_semver_into_string_and_back() {
    let s = "2.4.6-beta";
    let ver = SemVer::try_from(s).unwrap();
    let s2 = ver.clone().into();
    let ver2 = SemVer::try_from(s2.as_str()).unwrap();
    assert_eq!(ver, ver2);
}

#[test]
fn test_semver_try_from_u32_and_into_u32() {
    let ver = SemVer {
        major: 1,
        minor: 2,
        revision: 3,
        tag: "".to_string(),
    };
    let n: u32 = ver.clone().into();
    let ver2 = SemVer::try_from(n).unwrap();
    assert_eq!(ver.major, ver2.major);
    assert_eq!(ver.minor, ver2.minor);
    assert_eq!(ver.revision, ver2.revision);
}

#[test]
fn test_semver_error_display() {
    use std::fmt::Display;
    let err = SemVerError::ParseError;
    let s = format!("{}", err);
    assert!(s.contains("ParseError"));
}

#[test]
fn test_semver_security_bad_input() {
    // Security: try very large version numbers
    let s = "999.999.999-evil";
    let ver = SemVer::try_from(s).unwrap();
    assert_eq!(ver.major, 999);
    assert_eq!(ver.minor, 999);
    assert_eq!(ver.revision, 999);
    assert_eq!(ver.tag, "evil");
    // Try empty string
    let s = "";
    let err = SemVer::try_from(s);
    assert!(err.is_err());
}
