/// In-memory table storage

use crate::eplite::command::parser::{ColumnDefinition, CreateTableStatement};
use crate::eplite::error::{Error, Result};
use std::collections::HashMap;

/// Represents a row of data
pub type Row = Vec<String>;

/// Table definition
#[derive(Debug, Clone)]
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

	/// Update rows matching a condition (simplified - updates all for now)
	pub fn update(&mut self, _condition: &str, updates: &[(String, String)]) -> Result<usize> {
		let mut count = 0;
		
		// Find column indexes for updates
		let mut col_updates = Vec::new();
		for (col_name, value) in updates {
			if let Some(pos) = self.columns.iter().position(|c| &c.name == col_name) {
				col_updates.push((pos, value.clone()));
			}
		}

		// Update all rows (simplified)
		for row in &mut self.rows {
			for (col_idx, value) in &col_updates {
				if *col_idx < row.len() {
					row[*col_idx] = value.clone();
					count += 1;
				}
			}
		}

		Ok(if count > 0 { 1 } else { 0 })
	}

	/// Delete rows matching a condition (simplified - deletes all for now)
	pub fn delete(&mut self, _condition: Option<&str>) -> Result<usize> {
		let count = self.rows.len();
		self.rows.clear();
		Ok(count)
	}
}

/// In-memory storage manager
#[derive(Debug)]
pub struct StorageManager {
	tables: HashMap<String, Table>,
}

impl StorageManager {
	pub fn new() -> Self {
		StorageManager {
			tables: HashMap::new(),
		}
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
		Ok(())
	}

	/// Get a table
	pub fn get_table(&self, name: &str) -> Option<&Table> {
		self.tables.get(name)
	}

	/// Get a mutable table
	pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
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
			Ok(())
		} else {
			Err(Error::NotFound(format!("Table '{}' not found", name)))
		}
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

	fn create_test_table() -> Table {
		let columns = vec![
			ColumnDefinition {
				name: "id".to_string(),
				data_type: "INTEGER".to_string(),
				constraints: vec!["PRIMARY KEY".to_string()],
			},
			ColumnDefinition {
				name: "name".to_string(),
				data_type: "TEXT".to_string(),
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
				data_type: "INTEGER".to_string(),
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
}
