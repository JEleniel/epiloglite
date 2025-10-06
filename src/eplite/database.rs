/// Database connection and management

use crate::eplite::command::processor::{ExecutionResult, Processor};
use crate::eplite::constants::DEFAULT_PAGE_SIZE;
use crate::eplite::error::{Error, Result};

#[cfg(feature = "std")]
use crate::eplite::os::file::DefaultFile;
#[cfg(feature = "std")]
use crate::eplite::persistence::pager::{JournalMode, Pager};
#[cfg(feature = "std")]
use crate::eplite::persistence::wal::CheckpointMode;

use crate::eplite::storage::StorageManager;

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

/// Database connection
pub struct Database {
	path: String,
	processor: Processor,
	#[cfg(feature = "std")]
	journal_mode: JournalMode,
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
			#[cfg(feature = "std")]
			journal_mode: JournalMode::Rollback,
		})
	}

	/// Open or create a database with WAL mode
	#[cfg(feature = "std")]
	pub fn open_with_wal<P: AsRef<Path>>(path: P) -> Result<Self> {
		let path_str = path
			.as_ref()
			.to_str()
			.ok_or_else(|| Error::InvalidFormat("Invalid path".to_string()))?
			.to_string();

		if path_str == ":memory:" {
			return Err(Error::InvalidFormat(
				"WAL mode not supported for in-memory databases".to_string(),
			));
		}

		// File-based database with WAL
		let file = Box::new(DefaultFile::open(&path_str, true, true, true)?);
		
		// Create WAL file
		let wal_path = format!("{}-wal", path_str);
		let wal_file = Box::new(DefaultFile::open(&wal_path, true, true, true)?);
		
		let mut pager = Pager::with_file(DEFAULT_PAGE_SIZE, file)?;
		pager.set_journal_mode(JournalMode::Wal, Some(wal_file))?;
		
		let mut storage = StorageManager::with_pager(pager);
		
		// Load existing data from disk
		storage.load_from_disk()?;
		
		let processor = Processor::with_storage(storage);

		Ok(Database {
			path: path_str,
			processor,
			journal_mode: JournalMode::Wal,
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

	/// Get the current journal mode
	#[cfg(feature = "std")]
	pub fn journal_mode(&self) -> JournalMode {
		self.journal_mode
	}

	/// Perform a checkpoint (WAL mode only)
	#[cfg(feature = "std")]
	pub fn checkpoint(&mut self, mode: CheckpointMode) -> Result<()> {
		self.processor.checkpoint(mode)
	}

	/// Begin a transaction
	pub fn begin_transaction(&mut self) -> Result<()> {
		self.processor.begin_transaction()
	}

	/// Commit a transaction
	pub fn commit_transaction(&mut self) -> Result<()> {
		self.processor.commit_transaction()
	}

	/// Rollback a transaction
	pub fn rollback_transaction(&mut self) -> Result<()> {
		self.processor.rollback_transaction()
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

	#[test]
	#[cfg(feature = "std")]
	fn test_database_wal_mode() {
		let temp_dir = std::env::temp_dir();
		let db_path = temp_dir.join("test_wal_db.db");

		{
			let mut db = Database::open_with_wal(&db_path).unwrap();
			assert_eq!(db.journal_mode(), JournalMode::Wal);

			// Create a table
			db.execute("CREATE TABLE test (id INTEGER, value TEXT)").unwrap();
			
			// Begin transaction
			db.begin_transaction().unwrap();
			
			// Insert some data
			db.execute("INSERT INTO test VALUES (1, 'WAL')").unwrap();
			db.execute("INSERT INTO test VALUES (2, 'Mode')").unwrap();
			
			// Commit transaction
			db.commit_transaction().unwrap();

			// Query the data
			let result = db.execute("SELECT * FROM test").unwrap();
			match result {
				ExecutionResult::Select { rows, .. } => {
					assert_eq!(rows.len(), 2);
				}
				_ => panic!("Expected Select"),
			}
		}

		// Cleanup
		let _ = std::fs::remove_file(&db_path);
		let _ = std::fs::remove_file(format!("{}-wal", db_path.display()));
	}

	// Note: Transaction rollback at the database level requires more sophisticated
	// state management. Currently rollback works at the page level in WAL mode.
	// For complete rollback support, consider using explicit SAVEPOINT/ROLLBACK commands.

	#[test]
	#[cfg(feature = "std")]
	fn test_database_checkpoint() {
		let temp_dir = std::env::temp_dir();
		let db_path = temp_dir.join("test_checkpoint_db.db");

		{
			let mut db = Database::open_with_wal(&db_path).unwrap();
			
			db.execute("CREATE TABLE test (id INTEGER)").unwrap();
			
			db.begin_transaction().unwrap();
			db.execute("INSERT INTO test VALUES (1)").unwrap();
			db.commit_transaction().unwrap();
			
			// Perform checkpoint
			db.checkpoint(CheckpointMode::Full).unwrap();
		}

		// Cleanup
		let _ = std::fs::remove_file(&db_path);
		let _ = std::fs::remove_file(format!("{}-wal", db_path.display()));
	}
}
