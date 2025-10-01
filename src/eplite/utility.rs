/// Utility functions for string handling, type conversion, etc.

use crate::eplite::error::{Error, Result};

/// Convert a string to uppercase
pub fn to_uppercase(s: &str) -> String {
	s.to_uppercase()
}

/// Convert a string to lowercase
pub fn to_lowercase(s: &str) -> String {
	s.to_lowercase()
}

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

/// Parse an integer from a string
pub fn parse_int(s: &str) -> Result<i64> {
	s.parse::<i64>()
		.map_err(|_| Error::TypeMismatch(format!("Cannot parse '{}' as integer", s)))
}

/// Parse a float from a string
pub fn parse_float(s: &str) -> Result<f64> {
	s.parse::<f64>()
		.map_err(|_| Error::TypeMismatch(format!("Cannot parse '{}' as float", s)))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_uppercase() {
		assert_eq!(to_uppercase("hello"), "HELLO");
	}

	#[test]
	fn test_to_lowercase() {
		assert_eq!(to_lowercase("WORLD"), "world");
	}

	#[test]
	fn test_is_valid_identifier() {
		assert!(is_valid_identifier("table_name"));
		assert!(is_valid_identifier("_private"));
		assert!(is_valid_identifier("Column1"));
		assert!(!is_valid_identifier("123invalid"));
		assert!(!is_valid_identifier(""));
		assert!(!is_valid_identifier("with-dash"));
	}

	#[test]
	fn test_parse_int() {
		assert_eq!(parse_int("42").unwrap(), 42);
		assert_eq!(parse_int("-100").unwrap(), -100);
		assert!(parse_int("not a number").is_err());
	}

	#[test]
	fn test_parse_float() {
		assert_eq!(parse_float("3.14").unwrap(), 3.14);
		assert_eq!(parse_float("-2.5").unwrap(), -2.5);
		assert!(parse_float("not a number").is_err());
	}
}
