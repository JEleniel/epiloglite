/// SQL command processor - coordinates tokenization, parsing, and execution

use crate::eplite::command::parser::{AggregateFunction, ColumnSelection, Parser, Statement};
use crate::eplite::error::{Error, Result};
use crate::eplite::storage::StorageManager;

#[cfg(not(feature = "std"))]
use alloc::{format, string::{String, ToString}, vec, vec::Vec};

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
				// Check if this is a JOIN query
				if !stmt.joins.is_empty() {
					self.execute_join_select(&stmt)
				} else if let Some(table) = self.storage.get_table(&stmt.from) {
					// Check if this is an aggregate query
					let has_aggregates = stmt.columns.iter().any(|col| matches!(
						col,
						ColumnSelection::Aggregate { .. } | ColumnSelection::CountStar
					));

					if has_aggregates {
						// Process aggregate query (GROUP BY handled inside if present)
						self.execute_aggregate_select(table, &stmt)
					} else {
						// Regular SELECT - check for ORDER BY
						let rows: Vec<Vec<String>> = if let Some(order_cols) = &stmt.order_by {
							// ORDER BY present - use first column
							if !order_cols.is_empty() {
								table
									.select_ordered(stmt.where_clause.as_deref(), &order_cols[0], true)?
									.into_iter()
									.map(|row| row.iter().cloned().collect())
									.collect()
							} else {
								table
									.select(stmt.where_clause.as_deref())?
									.into_iter()
									.map(|row| row.iter().cloned().collect())
									.collect()
							}
						} else {
							// No ORDER BY
							table
								.select(stmt.where_clause.as_deref())?
								.into_iter()
								.map(|row| row.iter().cloned().collect())
								.collect()
						};
						
						// Extract column names for display
						let column_names: Vec<String> = stmt.columns.iter().map(|col| {
							match col {
								ColumnSelection::Column(name) => name.clone(),
								_ => "*".to_string(),
							}
						}).collect();
						
						Ok(ExecutionResult::Select {
							rows,
							columns: column_names,
						})
					}
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
			Statement::Savepoint(_name) => {
				// For now, savepoints are accepted but not enforced
				// Full implementation requires transaction state tracking
				Ok(ExecutionResult::Success)
			}
			Statement::Release(_name) => {
				// For now, releases are accepted but not enforced
				// Full implementation requires transaction state tracking
				Ok(ExecutionResult::Success)
			}
			Statement::RollbackToSavepoint(_name) => {
				// For now, rollback to savepoint is accepted but not enforced
				// Full implementation requires transaction state tracking
				Ok(ExecutionResult::Success)
			}
		}
	}

	/// Execute aggregate SELECT query
	fn execute_aggregate_select(
		&self,
		table: &crate::eplite::storage::Table,
		stmt: &crate::eplite::command::parser::SelectStatement,
	) -> Result<ExecutionResult> {
		// Check if GROUP BY is present
		if let Some(group_cols) = &stmt.group_by {
			if !group_cols.is_empty() {
				return self.execute_grouped_aggregate_select(table, stmt, &group_cols[0]);
			}
		}

		// Get filtered rows
		let rows = table.select(stmt.where_clause.as_deref())?;

		// Calculate aggregates
		let mut result_row = Vec::new();
		let mut column_names = Vec::new();

		for col_sel in &stmt.columns {
			match col_sel {
				ColumnSelection::CountStar => {
					result_row.push(rows.len().to_string());
					column_names.push("COUNT(*)".to_string());
				}
				ColumnSelection::Aggregate { function, column } => {
					// Find column index
					let col_idx = table
						.columns
						.iter()
						.position(|c| c.name == *column)
						.ok_or_else(|| Error::NotFound(format!("Column '{}' not found", column)))?;

					let result = match function {
						AggregateFunction::Count => {
							let count = rows.iter().filter(|r| col_idx < r.len()).count();
							count.to_string()
						}
						AggregateFunction::Sum => {
							let sum: f64 = rows
								.iter()
								.filter_map(|r| {
									if col_idx < r.len() {
										r[col_idx].parse::<f64>().ok()
									} else {
										None
									}
								})
								.sum();
			sum.to_string()
						}
						AggregateFunction::Avg => {
							let values: Vec<f64> = rows
								.iter()
								.filter_map(|r| {
									if col_idx < r.len() {
										r[col_idx].parse::<f64>().ok()
									} else {
										None
									}
								})
								.collect();
							if values.is_empty() {
								"0".to_string()
							} else {
								let avg = values.iter().sum::<f64>() / values.len() as f64;
								avg.to_string()
							}
						}
						AggregateFunction::Min => rows
							.iter()
							.filter_map(|r| {
								if col_idx < r.len() {
									Some(r[col_idx].clone())
								} else {
									None
								}
							})
							.min()
							.unwrap_or_else(|| "NULL".to_string()),
						AggregateFunction::Max => rows
							.iter()
							.filter_map(|r| {
								if col_idx < r.len() {
									Some(r[col_idx].clone())
								} else {
									None
								}
							})
							.max()
							.unwrap_or_else(|| "NULL".to_string()),
					};

					result_row.push(result);
					column_names.push(format!("{}({})", 
						match function {
							AggregateFunction::Count => "COUNT",
							AggregateFunction::Sum => "SUM",
							AggregateFunction::Avg => "AVG",
							AggregateFunction::Min => "MIN",
							AggregateFunction::Max => "MAX",
						},
						column
					));
				}
				ColumnSelection::Column(_) => {
					// Regular columns in aggregate query - not supported yet
					return Err(Error::Syntax(
						"Non-aggregate columns require GROUP BY".to_string(),
					));
				}
			}
		}

		Ok(ExecutionResult::Select {
			rows: vec![result_row],
			columns: column_names,
		})
	}

	/// Execute aggregates with GROUP BY
	fn execute_grouped_aggregate_select(
		&self,
		table: &crate::eplite::storage::Table,
		stmt: &crate::eplite::command::parser::SelectStatement,
		group_col: &str,
	) -> Result<ExecutionResult> {
		// Get grouped rows
		let groups = table.select_grouped(stmt.where_clause.as_deref(), group_col)?;

		let mut result_rows = Vec::new();
		let mut column_names = Vec::new();

		// Build column names first
		column_names.push(group_col.to_string());
		for col_sel in &stmt.columns {
			match col_sel {
				ColumnSelection::CountStar => {
					column_names.push("COUNT(*)".to_string());
				}
				ColumnSelection::Aggregate { function, column } => {
					column_names.push(format!("{}({})", 
						match function {
							AggregateFunction::Count => "COUNT",
							AggregateFunction::Sum => "SUM",
							AggregateFunction::Avg => "AVG",
							AggregateFunction::Min => "MIN",
							AggregateFunction::Max => "MAX",
						},
						column
					));
				}
				_ => {}
			}
		}

		// Process each group
		for (group_key, rows) in groups {
			let mut result_row = vec![group_key];

			for col_sel in &stmt.columns {
				match col_sel {
					ColumnSelection::CountStar => {
						result_row.push(rows.len().to_string());
					}
					ColumnSelection::Aggregate { function, column } => {
						// Find column index
						let col_idx = table
							.columns
							.iter()
							.position(|c| c.name == *column)
							.ok_or_else(|| Error::NotFound(format!("Column '{}' not found", column)))?;

						let result = match function {
							AggregateFunction::Count => {
								let count = rows.iter().filter(|r| col_idx < r.len()).count();
								count.to_string()
							}
							AggregateFunction::Sum => {
								let sum: f64 = rows
									.iter()
									.filter_map(|r| {
										if col_idx < r.len() {
											r[col_idx].parse::<f64>().ok()
										} else {
											None
										}
									})
									.sum();
								sum.to_string()
							}
							AggregateFunction::Avg => {
								let values: Vec<f64> = rows
									.iter()
									.filter_map(|r| {
										if col_idx < r.len() {
											r[col_idx].parse::<f64>().ok()
										} else {
											None
										}
									})
									.collect();
								if values.is_empty() {
									"0".to_string()
								} else {
									let avg = values.iter().sum::<f64>() / values.len() as f64;
									avg.to_string()
								}
							}
							AggregateFunction::Min => rows
								.iter()
								.filter_map(|r| {
									if col_idx < r.len() {
										Some(r[col_idx].clone())
									} else {
										None
									}
								})
								.min()
								.unwrap_or_else(|| "NULL".to_string()),
							AggregateFunction::Max => rows
								.iter()
								.filter_map(|r| {
									if col_idx < r.len() {
										Some(r[col_idx].clone())
									} else {
										None
									}
								})
								.max()
								.unwrap_or_else(|| "NULL".to_string()),
						};

						result_row.push(result);
					}
					_ => {}
				}
			}

			result_rows.push(result_row);
		}

		Ok(ExecutionResult::Select {
			rows: result_rows,
			columns: column_names,
		})
	}

	/// Execute SELECT with JOIN clauses
	fn execute_join_select(&self, stmt: &crate::eplite::command::parser::SelectStatement) -> Result<ExecutionResult> {
		use crate::eplite::command::parser::JoinType;

		// Currently we support single JOIN operations
		// TODO: Support multiple JOINs
		if stmt.joins.len() != 1 {
			return Err(Error::NotSupported("Multiple JOINs not yet supported".to_string()));
		}

		let join = &stmt.joins[0];
		let table1_name = &stmt.from;
		let table2_name = &join.table;

		// Execute the appropriate join
		let (rows, columns) = match join.join_type {
			JoinType::Cross => {
				self.storage.cross_join(table1_name, table2_name)?
			}
			JoinType::Inner => {
				let on_condition = join.on_condition.as_ref()
					.ok_or_else(|| Error::Syntax("INNER JOIN requires ON condition".to_string()))?;
				self.storage.inner_join(table1_name, table2_name, on_condition)?
			}
			JoinType::Left => {
				let on_condition = join.on_condition.as_ref()
					.ok_or_else(|| Error::Syntax("LEFT JOIN requires ON condition".to_string()))?;
				self.storage.left_join(table1_name, table2_name, on_condition)?
			}
			JoinType::Right => {
				let on_condition = join.on_condition.as_ref()
					.ok_or_else(|| Error::Syntax("RIGHT JOIN requires ON condition".to_string()))?;
				self.storage.right_join(table1_name, table2_name, on_condition)?
			}
		};

		// Apply WHERE clause if present
		let filtered_rows = if let Some(where_clause) = &stmt.where_clause {
			// For now, simple filtering - would need proper WHERE evaluation for joins
			// This is a simplified implementation
			rows
		} else {
			rows
		};

		Ok(ExecutionResult::Select {
			rows: filtered_rows,
			columns,
		})
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
