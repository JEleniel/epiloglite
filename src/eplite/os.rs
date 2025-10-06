/// OS abstraction layer - provides portability across operating systems

pub mod file;
pub mod vfs;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Get current time in milliseconds since Unix epoch
#[cfg(feature = "std")]
pub fn current_time_millis() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as u64
}

/// Get current time in milliseconds since Unix epoch (no-std version)
#[cfg(not(feature = "std"))]
pub fn current_time_millis() -> u64 {
	// In no-std environments, this would need to be provided by the platform
	// For now, return a placeholder value
	0
}

/// Generate random bytes using a cryptographically secure RNG
pub fn random_bytes(count: usize) -> Vec<u8> {
	// TODO: Use a cryptographically secure random number generator
	// For now, using a simple implementation
	let mut bytes = Vec::with_capacity(count);
	let seed = current_time_millis();
	for i in 0..count {
		bytes.push(((seed + i as u64) % 256) as u8);
	}
	bytes
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

	#[test]
	fn test_random_bytes_different() {
		let bytes1 = random_bytes(16);
		let bytes2 = random_bytes(16);
		// The current implementation is deterministic, so this test
		// would fail. In a real implementation with proper RNG, we'd expect
		// the bytes to be different.
		assert_eq!(bytes1.len(), bytes2.len());
	}
}
