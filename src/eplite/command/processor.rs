/// SQL command processor - coordinates tokenization, parsing, and execution

use crate::eplite::command::parser::{Parser, Statement};
use crate::eplite::error::{Error, Result};
use crate::eplite::storage::StorageManager;

/// Processes SQL commands
#[derive(Debug)]
pub struct Processor {
	parser: Parser,
	storage: StorageManager,
}

impl Processor {
	pub fn new() -> Self {
		Processor {
			parser: Parser::new(),
			storage: StorageManager::new(),
		}
	}

	/// Create a processor with a specific storage manager
	pub fn with_storage(storage: StorageManager) -> Self {
		Processor {
			parser: Parser::new(),
			storage,
		}
	}

	/// Flush any pending changes to disk
	pub fn flush(&mut self) -> Result<()> {
		self.storage.flush()
	}

	/// Execute a SQL statement
	pub fn execute(&mut self, sql: &str) -> Result<ExecutionResult> {
		// Parse the SQL
		let statement = self.parser.parse(sql)?;

		// Execute based on statement type
		match statement {
			Statement::Select(stmt) => {
				// Get the table
				if let Some(table) = self.storage.get_table(&stmt.from) {
					// Use the new select method with WHERE clause support
					let rows: Vec<Vec<String>> = table
						.select(stmt.where_clause.as_deref())?
						.into_iter()
						.map(|row| row.iter().cloned().collect())
						.collect();
					
					Ok(ExecutionResult::Select {
						rows,
						columns: stmt.columns,
					})
				} else {
					Err(Error::NotFound(format!("Table '{}' not found", stmt.from)))
				}
			}
			Statement::Insert(stmt) => {
				// Get the table
				if let Some(table) = self.storage.get_table_mut(&stmt.table) {
					table.insert(stmt.values)?;
					// Flush to disk after insert
					self.storage.flush()?;
					Ok(ExecutionResult::RowsAffected(1))
				} else {
					Err(Error::NotFound(format!("Table '{}' not found", stmt.table)))
				}
			}
			Statement::Update(stmt) => {
				// Get the table
				if let Some(table) = self.storage.get_table_mut(&stmt.table) {
					let count = table.update(
						stmt.where_clause.as_deref(),
						&stmt.set_clauses,
					)?;
					// Flush to disk after update
					self.storage.flush()?;
					Ok(ExecutionResult::RowsAffected(count))
				} else {
					Err(Error::NotFound(format!("Table '{}' not found", stmt.table)))
				}
			}
			Statement::Delete(stmt) => {
				// Get the table
				if let Some(table) = self.storage.get_table_mut(&stmt.table) {
					let count = table.delete(stmt.where_clause.as_deref())?;
					// Flush to disk after delete
					self.storage.flush()?;
					Ok(ExecutionResult::RowsAffected(count))
				} else {
					Err(Error::NotFound(format!("Table '{}' not found", stmt.table)))
				}
			}
			Statement::CreateTable(stmt) => {
				self.storage.create_table(stmt)?;
				Ok(ExecutionResult::Success)
			}
			Statement::BeginTransaction => Ok(ExecutionResult::Success),
			Statement::Commit => Ok(ExecutionResult::Success),
			Statement::Rollback => Ok(ExecutionResult::Success),
		}
	}
}

impl Default for Processor {
	fn default() -> Self {
		Self::new()
	}
}

/// Result of executing a SQL statement
#[derive(Debug, Clone)]
pub enum ExecutionResult {
	/// SELECT statement result
	Select {
		rows: Vec<Vec<String>>,
		columns: Vec<String>,
	},
	/// Number of rows affected by INSERT/UPDATE/DELETE
	RowsAffected(usize),
	/// Statement executed successfully
	Success,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_processor_creation() {
		let processor = Processor::new();
		assert!(format!("{:?}", processor).contains("Processor"));
	}

	#[test]
	fn test_execute_select() {
		let mut processor = Processor::new();
		// First create the table
		processor
			.execute("CREATE TABLE users (id INTEGER, name TEXT)")
			.unwrap();
		// Then query it
		let result = processor.execute("SELECT * FROM users");
		if let Err(e) = &result {
			eprintln!("Error: {}", e);
		}
		assert!(result.is_ok());
		match result.unwrap() {
			ExecutionResult::Select { columns, .. } => {
				assert_eq!(columns.len(), 1);
			}
			_ => panic!("Expected Select result"),
		}
	}

	#[test]
	fn test_execute_insert() {
		let mut processor = Processor::new();
		// First create the table
		processor
			.execute("CREATE TABLE users (id INTEGER, name TEXT)")
			.unwrap();
		// Then insert
		let result = processor.execute("INSERT INTO users VALUES (1, 'John')");
		assert!(result.is_ok());
		match result.unwrap() {
			ExecutionResult::RowsAffected(n) => {
				assert_eq!(n, 1);
			}
			_ => panic!("Expected RowsAffected result"),
		}
	}

	#[test]
	fn test_execute_create_table() {
		let mut processor = Processor::new();
		let result = processor.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), ExecutionResult::Success));
	}

	#[test]
	fn test_execute_transactions() {
		let mut processor = Processor::new();
		
		let result = processor.execute("BEGIN");
		assert!(result.is_ok());
		
		let result = processor.execute("COMMIT");
		assert!(result.is_ok());
		
		let result = processor.execute("ROLLBACK");
		assert!(result.is_ok());
	}

	#[test]
	fn test_execute_invalid_sql() {
		let mut processor = Processor::new();
		let result = processor.execute("INVALID SQL");
		assert!(result.is_err());
	}
}
