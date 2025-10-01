use std::fmt;
use std::io;

/// Result type for EpilogLite operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for EpilogLite
#[derive(Debug)]
pub enum Error {
	/// I/O error
	Io(io::Error),
	/// File format error
	InvalidFormat(String),
	/// SQL syntax error
	Syntax(String),
	/// Database is locked
	Locked,
	/// Database is busy
	Busy,
	/// Constraint violation
	Constraint(String),
	/// Type mismatch
	TypeMismatch(String),
	/// Not found
	NotFound(String),
	/// Permission denied
	PermissionDenied(String),
	/// Database is corrupt
	Corrupt(String),
	/// Internal error
	Internal(String),
	/// Not supported
	NotSupported(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Io(e) => write!(f, "I/O error: {}", e),
			Error::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
			Error::Syntax(msg) => write!(f, "Syntax error: {}", msg),
			Error::Locked => write!(f, "Database is locked"),
			Error::Busy => write!(f, "Database is busy"),
			Error::Constraint(msg) => write!(f, "Constraint violation: {}", msg),
			Error::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
			Error::NotFound(msg) => write!(f, "Not found: {}", msg),
			Error::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
			Error::Corrupt(msg) => write!(f, "Database is corrupt: {}", msg),
			Error::Internal(msg) => write!(f, "Internal error: {}", msg),
			Error::NotSupported(msg) => write!(f, "Not supported: {}", msg),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Io(e) => Some(e),
			_ => None,
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::Io(err)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_error_display() {
		let error = Error::Syntax("unexpected token".to_string());
		assert_eq!(format!("{}", error), "Syntax error: unexpected token");
	}

	#[test]
	fn test_io_error_conversion() {
		let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
		let err: Error = io_err.into();
		assert!(matches!(err, Error::Io(_)));
	}
}
