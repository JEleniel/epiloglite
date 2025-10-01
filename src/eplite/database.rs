/// Database connection and management

use crate::eplite::command::processor::{ExecutionResult, Processor};
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
	pub fn execute(&mut self, sql: &str) -> Result<ExecutionResult> {
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
	fn test_database_execute_select() {
		let mut db = Database::open(":memory:").unwrap();
		let result = db.execute("SELECT * FROM users");
		assert!(result.is_ok());
	}

	#[test]
	fn test_database_execute_create() {
		let mut db = Database::open(":memory:").unwrap();
		let result = db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), ExecutionResult::Success));
	}

	#[test]
	fn test_database_execute_insert() {
		let mut db = Database::open(":memory:").unwrap();
		let result = db.execute("INSERT INTO users VALUES (1, 'John')");
		assert!(result.is_ok());
		match result.unwrap() {
			ExecutionResult::RowsAffected(n) => assert_eq!(n, 1),
			_ => panic!("Expected RowsAffected"),
		}
	}
}
