/// SQL command processor - coordinates tokenization, parsing, and execution

use crate::eplite::error::Result;

/// Processes SQL commands
#[derive(Debug)]
pub struct Processor {}

impl Processor {
	pub fn new() -> Self {
		Processor {}
	}

	/// Execute a SQL statement
	pub fn execute(&mut self, _sql: &str) -> Result<()> {
		// TODO: Implement SQL execution
		Err(crate::eplite::error::Error::NotSupported(
			"SQL execution not yet implemented".to_string(),
		))
	}
}

impl Default for Processor {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_processor_creation() {
		let processor = Processor::new();
		assert!(format!("{:?}", processor).contains("Processor"));
	}
}
