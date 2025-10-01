/// In-memory table storage with disk persistence support

use crate::eplite::command::parser::{ColumnDefinition, CreateTableStatement};
use crate::eplite::error::{Error, Result};
use crate::eplite::persistence::pager::Pager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WHERE clause evaluator
mod where_clause {
	use super::*;

	/// Comparison operators
	#[derive(Debug, Clone, PartialEq)]
	pub enum CompOp {
		Equal,
		NotEqual,
		LessThan,
		GreaterThan,
		LessOrEqual,
		GreaterOrEqual,
		Like,
	}

	/// WHERE clause condition
	#[derive(Debug, Clone)]
	pub struct Condition {
		pub column: String,
		pub operator: CompOp,
		pub value: String,
	}

	impl Condition {
		/// Parse a simple WHERE clause (e.g., "id = 1", "name = 'Alice'", "age > 25")
		pub fn parse(clause: &str) -> Result<Self> {
			let clause = clause.trim();

			// Try each operator in order of length (to match >= before >)
			let operators = [
				(">=", CompOp::GreaterOrEqual),
				("<=", CompOp::LessOrEqual),
				("<>", CompOp::NotEqual),
				("!=", CompOp::NotEqual),
				("=", CompOp::Equal),
				("<", CompOp::LessThan),
				(">", CompOp::GreaterThan),
				(" LIKE ", CompOp::Like),
			];

			for (op_str, op) in &operators {
				if let Some(pos) = clause.find(op_str) {
					let column = clause[..pos].trim().to_string();
					let value = clause[pos + op_str.len()..].trim().to_string();

					if column.is_empty() || value.is_empty() {
						return Err(Error::Syntax(format!("Invalid WHERE clause: {}", clause)));
					}

					return Ok(Condition {
						column,
						operator: op.clone(),
						value,
					});
				}
			}

			Err(Error::Syntax(format!("Invalid WHERE clause: {}", clause)))
		}

		/// Evaluate condition against a row
		pub fn evaluate(&self, row: &[String], columns: &[ColumnDefinition]) -> bool {
			// Find column index
			let col_idx = match columns.iter().position(|c| c.name == self.column) {
				Some(idx) => idx,
				None => return false,
			};

			if col_idx >= row.len() {
				return false;
			}

			let row_value = &row[col_idx];
			let compare_value = self.value.trim_matches('\'').trim_matches('"');

			match self.operator {
				CompOp::Equal => row_value == compare_value,
				CompOp::NotEqual => row_value != compare_value,
				CompOp::LessThan => {
					// Try numeric comparison first
					if let (Ok(a), Ok(b)) = (row_value.parse::<f64>(), compare_value.parse::<f64>()) {
						a < b
					} else {
						row_value.as_str() < compare_value
					}
				}
				CompOp::GreaterThan => {
					if let (Ok(a), Ok(b)) = (row_value.parse::<f64>(), compare_value.parse::<f64>()) {
						a > b
					} else {
						row_value.as_str() > compare_value
					}
				}
				CompOp::LessOrEqual => {
					if let (Ok(a), Ok(b)) = (row_value.parse::<f64>(), compare_value.parse::<f64>()) {
						a <= b
					} else {
						row_value.as_str() <= compare_value
					}
				}
				CompOp::GreaterOrEqual => {
					if let (Ok(a), Ok(b)) = (row_value.parse::<f64>(), compare_value.parse::<f64>()) {
						a >= b
					} else {
						row_value.as_str() >= compare_value
					}
				}
				CompOp::Like => {
					// Simple LIKE implementation (% = wildcard)
					let pattern = compare_value.replace('%', ".*");
					if let Ok(re) = regex::Regex::new(&format!("^{}$", pattern)) {
						re.is_match(row_value)
					} else {
						false
					}
				}
			}
		}
	}
}

/// Represents a row of data
pub type Row = Vec<String>;

/// Table definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
	pub name: String,
	pub columns: Vec<ColumnDefinition>,
	pub rows: Vec<Row>,
}

impl Table {
	pub fn new(name: String, columns: Vec<ColumnDefinition>) -> Self {
		Table {
			name,
			columns,
			rows: Vec::new(),
		}
	}

	/// Get the number of rows
	pub fn row_count(&self) -> usize {
		self.rows.len()
	}

	/// Insert a row
	pub fn insert(&mut self, row: Row) -> Result<()> {
		// Validate row length
		if row.len() != self.columns.len() && !row.is_empty() {
			return Err(Error::Constraint(format!(
				"Column count mismatch: expected {}, got {}",
				self.columns.len(),
				row.len()
			)));
		}
		
		self.rows.push(row);
		Ok(())
	}

	/// Select all rows
	pub fn select_all(&self) -> Vec<Row> {
		self.rows.clone()
	}

	/// Select rows with WHERE clause filtering
	pub fn select(&self, where_clause: Option<&str>) -> Result<Vec<Row>> {
		// If no WHERE clause, return all rows
		let Some(clause) = where_clause else {
			return Ok(self.rows.clone());
		};

		// Parse and evaluate WHERE clause
		let condition = where_clause::Condition::parse(clause)?;
		let filtered: Vec<Row> = self
			.rows
			.iter()
			.filter(|row| condition.evaluate(row, &self.columns))
			.cloned()
			.collect();

		Ok(filtered)
	}

	/// Update rows matching a condition
	pub fn update(&mut self, condition: Option<&str>, updates: &[(String, String)]) -> Result<usize> {
		let mut count = 0;
		
		// Parse WHERE clause if provided
		let where_cond = if let Some(clause) = condition {
			Some(where_clause::Condition::parse(clause)?)
		} else {
			None
		};
		
		// Find column indexes for updates
		let mut col_updates = Vec::new();
		for (col_name, value) in updates {
			if let Some(pos) = self.columns.iter().position(|c| &c.name == col_name) {
				col_updates.push((pos, value.clone()));
			}
		}

		// Update matching rows
		for row in &mut self.rows {
			// Check if row matches WHERE clause
			if let Some(ref cond) = where_cond {
				if !cond.evaluate(row, &self.columns) {
					continue;
				}
			}

			// Update the row
			for (col_idx, value) in &col_updates {
				if *col_idx < row.len() {
					row[*col_idx] = value.clone();
				}
			}
			count += 1;
		}

		Ok(count)
	}

	/// Delete rows matching a condition
	pub fn delete(&mut self, condition: Option<&str>) -> Result<usize> {
		// Parse WHERE clause if provided
		let where_cond = if let Some(clause) = condition {
			Some(where_clause::Condition::parse(clause)?)
		} else {
			None
		};

		let original_count = self.rows.len();

		// Filter out rows that match the condition
		if let Some(cond) = where_cond {
			self.rows.retain(|row| !cond.evaluate(row, &self.columns));
		} else {
			// No WHERE clause means delete all
			self.rows.clear();
		}

		Ok(original_count - self.rows.len())
	}
}

/// Storage manager with optional disk persistence
#[derive(Debug)]
pub struct StorageManager {
	tables: HashMap<String, Table>,
	pager: Option<Pager>,
	dirty: bool,
}

impl StorageManager {
	pub fn new() -> Self {
		StorageManager {
			tables: HashMap::new(),
			pager: None,
			dirty: false,
		}
	}

	/// Create a storage manager with disk persistence
	pub fn with_pager(pager: Pager) -> Self {
		StorageManager {
			tables: HashMap::new(),
			pager: Some(pager),
			dirty: false,
		}
	}

	/// Load tables from disk if pager is available
	pub fn load_from_disk(&mut self) -> Result<()> {
		if let Some(pager) = &mut self.pager {
			// Try to load from page 1 (page 0 is header)
			if let Ok(page) = pager.get_page(1) {
				// Deserialize the tables from the page data
				if !page.data.is_empty() && page.data[0] != 0 {
					match bincode::deserialize::<HashMap<String, Table>>(&page.data) {
						Ok(tables) => {
							self.tables = tables;
							return Ok(());
						}
						Err(_) => {
							// Page exists but can't deserialize - might be empty/new database
						}
					}
				}
			}
		}
		Ok(())
	}

	/// Save tables to disk if pager is available
	pub fn save_to_disk(&mut self) -> Result<()> {
		if self.dirty && self.pager.is_some() {
			if let Some(pager) = &mut self.pager {
				// Serialize the tables
				let serialized = bincode::serialize(&self.tables).map_err(|e| {
					Error::Internal(format!("Failed to serialize tables: {}", e))
				})?;

				// Get or create page 1
				let page = pager.get_page_mut(1)?;
				
				// Write serialized data
				if serialized.len() <= page.data.len() {
					page.data[..serialized.len()].copy_from_slice(&serialized);
					page.mark_dirty();
				} else {
					return Err(Error::Internal(format!(
						"Serialized data too large: {} bytes (max: {})",
						serialized.len(),
						page.data.len()
					)));
				}

				// Flush to disk
				pager.flush()?;
				self.dirty = false;
			}
		}
		Ok(())
	}

	/// Mark storage as dirty (needs save)
	fn mark_dirty(&mut self) {
		self.dirty = true;
	}

	/// Create a table
	pub fn create_table(&mut self, stmt: CreateTableStatement) -> Result<()> {
		if self.tables.contains_key(&stmt.name) {
			return Err(Error::Constraint(format!(
				"Table '{}' already exists",
				stmt.name
			)));
		}

		let table = Table::new(stmt.name.clone(), stmt.columns);
		self.tables.insert(stmt.name, table);
		self.mark_dirty();
		self.save_to_disk()?;
		Ok(())
	}

	/// Get a table
	pub fn get_table(&self, name: &str) -> Option<&Table> {
		self.tables.get(name)
	}

	/// Get a mutable table
	pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
		if self.tables.contains_key(name) {
			self.mark_dirty();
		}
		self.tables.get_mut(name)
	}

	/// Check if a table exists
	pub fn table_exists(&self, name: &str) -> bool {
		self.tables.contains_key(name)
	}

	/// List all table names
	pub fn list_tables(&self) -> Vec<String> {
		self.tables.keys().cloned().collect()
	}

	/// Drop a table
	pub fn drop_table(&mut self, name: &str) -> Result<()> {
		if self.tables.remove(name).is_some() {
			self.mark_dirty();
			self.save_to_disk()?;
			Ok(())
		} else {
			Err(Error::NotFound(format!("Table '{}' not found", name)))
		}
	}

	/// Flush any pending changes to disk
	pub fn flush(&mut self) -> Result<()> {
		self.save_to_disk()
	}
}

impl Default for StorageManager {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::eplite::types::column::ColumnType;

	fn create_test_table() -> Table {
		let columns = vec![
			ColumnDefinition {
				name: "id".to_string(),
				data_type: ColumnType::Int32,
				constraints: vec!["PRIMARY KEY".to_string()],
			},
			ColumnDefinition {
				name: "name".to_string(),
				data_type: ColumnType::Text,
				constraints: vec![],
			},
		];
		Table::new("users".to_string(), columns)
	}

	#[test]
	fn test_table_creation() {
		let table = create_test_table();
		assert_eq!(table.name, "users");
		assert_eq!(table.columns.len(), 2);
		assert_eq!(table.row_count(), 0);
	}

	#[test]
	fn test_table_insert() {
		let mut table = create_test_table();
		let row = vec!["1".to_string(), "Alice".to_string()];
		table.insert(row).unwrap();
		assert_eq!(table.row_count(), 1);
	}

	#[test]
	fn test_table_select_all() {
		let mut table = create_test_table();
		table
			.insert(vec!["1".to_string(), "Alice".to_string()])
			.unwrap();
		table
			.insert(vec!["2".to_string(), "Bob".to_string()])
			.unwrap();
		
		let rows = table.select_all();
		assert_eq!(rows.len(), 2);
	}

	#[test]
	fn test_storage_manager() {
		let mut mgr = StorageManager::new();
		
		let stmt = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![ColumnDefinition {
				name: "id".to_string(),
				data_type: ColumnType::Int32,
				constraints: vec![],
			}],
		};

		mgr.create_table(stmt).unwrap();
		assert!(mgr.table_exists("users"));
	}

	#[test]
	fn test_storage_manager_duplicate_table() {
		let mut mgr = StorageManager::new();
		
		let stmt = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};

		mgr.create_table(stmt.clone()).unwrap();
		let result = mgr.create_table(stmt);
		assert!(result.is_err());
	}

	#[test]
	fn test_storage_manager_list_tables() {
		let mut mgr = StorageManager::new();
		
		let stmt1 = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};
		let stmt2 = CreateTableStatement {
			name: "posts".to_string(),
			columns: vec![],
		};

		mgr.create_table(stmt1).unwrap();
		mgr.create_table(stmt2).unwrap();
		
		let tables = mgr.list_tables();
		assert_eq!(tables.len(), 2);
		assert!(tables.contains(&"users".to_string()));
		assert!(tables.contains(&"posts".to_string()));
	}

	#[test]
	fn test_where_clause_equal() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		table.insert(vec!["3".to_string(), "Charlie".to_string()]).unwrap();

		let rows = table.select(Some("id = 2")).unwrap();
		assert_eq!(rows.len(), 1);
		assert_eq!(rows[0][0], "2");
		assert_eq!(rows[0][1], "Bob");
	}

	#[test]
	fn test_where_clause_greater_than() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		table.insert(vec!["3".to_string(), "Charlie".to_string()]).unwrap();

		let rows = table.select(Some("id > 1")).unwrap();
		assert_eq!(rows.len(), 2);
	}

	#[test]
	fn test_where_clause_less_than() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		table.insert(vec!["3".to_string(), "Charlie".to_string()]).unwrap();

		let rows = table.select(Some("id < 3")).unwrap();
		assert_eq!(rows.len(), 2);
	}

	#[test]
	fn test_where_clause_string_equal() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();

		let rows = table.select(Some("name = 'Alice'")).unwrap();
		assert_eq!(rows.len(), 1);
		assert_eq!(rows[0][1], "Alice");
	}

	#[test]
	fn test_update_with_where() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();

		let updates = vec![("name".to_string(), "Bobby".to_string())];
		let count = table.update(Some("id = 2"), &updates).unwrap();
		assert_eq!(count, 1);

		let rows = table.select(Some("id = 2")).unwrap();
		assert_eq!(rows[0][1], "Bobby");
	}

	#[test]
	fn test_delete_with_where() {
		let mut table = create_test_table();
		table.insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		table.insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		table.insert(vec!["3".to_string(), "Charlie".to_string()]).unwrap();

		let count = table.delete(Some("id = 2")).unwrap();
		assert_eq!(count, 1);
		assert_eq!(table.row_count(), 2);

		// Verify Bob is gone
		let rows = table.select(None).unwrap();
		assert!(!rows.iter().any(|r| r[1] == "Bob"));
	}
}
