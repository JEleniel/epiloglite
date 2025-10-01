/// Database connection and management

use crate::eplite::command::processor::Processor;
use crate::eplite::error::{Error, Result};
use std::path::Path;

/// Database connection
pub struct Database {
	path: String,
	processor: Processor,
}

impl Database {
	/// Open or create a database
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		let path_str = path
			.as_ref()
			.to_str()
			.ok_or_else(|| Error::InvalidFormat("Invalid path".to_string()))?
			.to_string();

		Ok(Database {
			path: path_str,
			processor: Processor::new(),
		})
	}

	/// Execute a SQL statement
	pub fn execute(&mut self, sql: &str) -> Result<()> {
		self.processor.execute(sql)
	}

	/// Get the database file path
	pub fn path(&self) -> &str {
		&self.path
	}

	/// Close the database connection
	pub fn close(self) -> Result<()> {
		// TODO: Flush any pending writes, close files
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_database_open() {
		let db = Database::open(":memory:");
		assert!(db.is_ok());
		let db = db.unwrap();
		assert_eq!(db.path(), ":memory:");
	}

	#[test]
	fn test_database_execute() {
		let mut db = Database::open(":memory:").unwrap();
		let result = db.execute("SELECT 1");
		assert!(result.is_err()); // Not yet implemented
	}
}
