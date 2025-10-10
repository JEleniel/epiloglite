/// Utility functions for string handling, type conversion, etc.

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

/// Check if a string is a valid identifier
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    if !(first.is_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use crate::eplite::utility::is_valid_identifier;

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("table_name"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("Column1"));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("with-dash"));
    }
}
