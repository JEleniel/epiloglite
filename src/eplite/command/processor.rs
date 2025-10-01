/// SQL command processor - coordinates tokenization, parsing, and execution

use crate::eplite::command::parser::{Parser, Statement};
use crate::eplite::error::{Error, Result};

/// Processes SQL commands
#[derive(Debug)]
pub struct Processor {
	parser: Parser,
}

impl Processor {
	pub fn new() -> Self {
		Processor {
			parser: Parser::new(),
		}
	}

	/// Execute a SQL statement
	pub fn execute(&mut self, sql: &str) -> Result<ExecutionResult> {
		// Parse the SQL
		let statement = self.parser.parse(sql)?;

		// Execute based on statement type
		match statement {
			Statement::Select(stmt) => {
				// For now, return a success result
				Ok(ExecutionResult::Select {
					rows: Vec::new(),
					columns: stmt.columns,
				})
			}
			Statement::Insert(_) => Ok(ExecutionResult::RowsAffected(1)),
			Statement::Update(_) => Ok(ExecutionResult::RowsAffected(1)),
			Statement::Delete(_) => Ok(ExecutionResult::RowsAffected(1)),
			Statement::CreateTable(_) => Ok(ExecutionResult::Success),
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
		let result = processor.execute("SELECT * FROM users");
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
