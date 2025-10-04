/// Database connection and management

use crate::eplite::command::processor::{ExecutionResult, Processor};
use crate::eplite::constants::DEFAULT_PAGE_SIZE;
use crate::eplite::error::{Error, Result};

#[cfg(feature = "std")]
use crate::eplite::os::file::DefaultFile;
#[cfg(feature = "std")]
use crate::eplite::persistence::pager::Pager;

use crate::eplite::storage::StorageManager;

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

/// Database connection
pub struct Database {
	path: String,
	processor: Processor,
}

impl Database {
	/// Open or create a database
	#[cfg(feature = "std")]
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		let path_str = path
			.as_ref()
			.to_str()
			.ok_or_else(|| Error::InvalidFormat("Invalid path".to_string()))?
			.to_string();

		// Create processor with disk-backed storage if not in-memory
		let processor = if path_str == ":memory:" {
			// In-memory database
			Processor::new()
		} else {
			// File-based database
			let file = Box::new(DefaultFile::open(&path_str, true, true, true)?);
			let pager = Pager::with_file(DEFAULT_PAGE_SIZE, file)?;
			let mut storage = StorageManager::with_pager(pager);
			
			// Load existing data from disk
			storage.load_from_disk()?;
			
			Processor::with_storage(storage)
		};

		Ok(Database {
			path: path_str,
			processor,
		})
	}

	/// Create a new in-memory database (no-std version)
	#[cfg(not(feature = "std"))]
	pub fn new() -> Result<Self> {
		Ok(Database {
			path: ":memory:".to_string(),
			processor: Processor::new(),
		})
	}

	/// Execute a SQL statement
	pub fn execute(&mut self, sql: &str) -> Result<ExecutionResult> {
		self.processor.execute(sql)
	}

	/// Execute a query from a builder
	pub fn execute_builder<B>(&mut self, builder: B) -> Result<ExecutionResult>
	where
		B: crate::eplite::query_builder::QueryBuilder,
	{
		let sql = builder.build()?;
		self.execute(&sql)
	}

	/// Get the database file path
	pub fn path(&self) -> &str {
		&self.path
	}

	/// Close the database connection
	pub fn close(mut self) -> Result<()> {
		// Flush any pending writes
		self.processor.flush()?;
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
		// First create the table
		db.execute("CREATE TABLE users (id INTEGER, name TEXT)")
			.unwrap();
		// Then query it
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
		// First create the table
		db.execute("CREATE TABLE users (id INTEGER, name TEXT)")
			.unwrap();
		// Then insert
		let result = db.execute("INSERT INTO users VALUES (1, 'John')");
		assert!(result.is_ok());
		match result.unwrap() {
			ExecutionResult::RowsAffected(n) => assert_eq!(n, 1),
			_ => panic!("Expected RowsAffected"),
		}
	}
}
