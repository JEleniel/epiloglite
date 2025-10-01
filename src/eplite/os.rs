/// OS abstraction layer - provides portability across operating systems

pub mod vfs;

use crate::eplite::error::Result;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current time in milliseconds since Unix epoch
pub fn current_time_millis() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as u64
}

/// Generate random bytes
pub fn random_bytes(count: usize) -> Vec<u8> {
	// TODO: Use a cryptographically secure random number generator
	vec![0; count]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_current_time() {
		let time = current_time_millis();
		assert!(time > 0);
	}

	#[test]
	fn test_random_bytes() {
		let bytes = random_bytes(16);
		assert_eq!(bytes.len(), 16);
	}
}
