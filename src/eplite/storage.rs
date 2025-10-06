/// In-memory table storage with disk persistence support

use crate::eplite::command::parser::{ColumnDefinition, CreateTableStatement};
use crate::eplite::error::{Error, Result};
use crate::eplite::persistence::pager::Pager;
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
	collections::BTreeMap as HashMap,
	format,
	string::{String, ToString},
	vec::Vec,
};

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

/// View definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
	pub name: String,
	pub query: String,
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

	/// Select rows with ORDER BY support
	pub fn select_ordered(&self, where_clause: Option<&str>, order_by_column: &str, ascending: bool) -> Result<Vec<Row>> {
		let mut rows = self.select(where_clause)?;
		
		// Find column index
		let col_index = self.columns.iter()
			.position(|c| c.name == order_by_column)
			.ok_or_else(|| Error::NotFound(format!("Column '{}' not found", order_by_column)))?;
		
		// Sort rows by the column
		rows.sort_by(|a, b| {
			let val_a = &a[col_index];
			let val_b = &b[col_index];
			
			// Try numeric comparison first
			if let (Ok(num_a), Ok(num_b)) = (val_a.parse::<f64>(), val_b.parse::<f64>()) {
				let cmp = num_a.partial_cmp(&num_b).unwrap_or(core::cmp::Ordering::Equal);
				if ascending { cmp } else { cmp.reverse() }
			} else {
				// String comparison
				let cmp = val_a.cmp(val_b);
				if ascending { cmp } else { cmp.reverse() }
			}
		});
		
		Ok(rows)
	}

	/// Select rows grouped by a column
	pub fn select_grouped(&self, where_clause: Option<&str>, group_by_column: &str) -> Result<HashMap<String, Vec<Row>>> {
		let rows = self.select(where_clause)?;
		
		// Find column index
		let col_index = self.columns.iter()
			.position(|c| c.name == group_by_column)
			.ok_or_else(|| Error::NotFound(format!("Column '{}' not found", group_by_column)))?;
		
		// Group rows by column value
		let mut groups = HashMap::new();
		for row in rows {
			let key = row[col_index].clone();
			groups.entry(key).or_insert_with(Vec::new).push(row);
		}
		
		Ok(groups)
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
	procedures: crate::eplite::procedures::ProcedureRegistry,
	views: HashMap<String, View>,
	triggers: HashMap<String, crate::eplite::command::parser::CreateTriggerStatement>,
	pager: Option<Pager>,
	dirty: bool,
}

impl StorageManager {
	pub fn new() -> Self {
		StorageManager {
			tables: HashMap::new(),
			procedures: crate::eplite::procedures::ProcedureRegistry::new(),
			views: HashMap::new(),
			triggers: HashMap::new(),
			pager: None,
			dirty: false,
		}
	}

	/// Create a storage manager with disk persistence
	pub fn with_pager(pager: Pager) -> Self {
		StorageManager {
			tables: HashMap::new(),
			procedures: crate::eplite::procedures::ProcedureRegistry::new(),
			views: HashMap::new(),
			triggers: HashMap::new(),
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
					// Try to deserialize as a tuple of (tables, views)
					match bincode::deserialize::<(HashMap<String, Table>, HashMap<String, View>)>(&page.data) {
						Ok((tables, views)) => {
							self.tables = tables;
							self.views = views;
							return Ok(());
						}
						Err(_) => {
							// Try legacy format (just tables)
							match bincode::deserialize::<HashMap<String, Table>>(&page.data) {
								Ok(tables) => {
									self.tables = tables;
									self.views = HashMap::new();
									return Ok(());
								}
								Err(_) => {
									// Page exists but can't deserialize - might be empty/new database
								}
							}
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
				// Serialize both tables and views
				let serialized = bincode::serialize(&(&self.tables, &self.views)).map_err(|e| {
					Error::Internal(format!("Failed to serialize tables and views: {}", e))
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

	/// Get procedure registry (immutable)
	pub fn get_procedures(&self) -> &crate::eplite::procedures::ProcedureRegistry {
		&self.procedures
	}

	/// Get procedure registry (mutable)
	pub fn get_procedures_mut(&mut self) -> &mut crate::eplite::procedures::ProcedureRegistry {
		&mut self.procedures
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

	/// Create a view
	pub fn create_view(&mut self, name: String, query: String) -> Result<()> {
		if self.views.contains_key(&name) {
			return Err(Error::Constraint(format!(
				"View '{}' already exists",
				name
			)));
		}
		
		// Check that the view name doesn't conflict with a table
		if self.tables.contains_key(&name) {
			return Err(Error::Constraint(format!(
				"A table named '{}' already exists",
				name
			)));
		}

		let view = View { name: name.clone(), query };
		self.views.insert(name, view);
		self.mark_dirty();
		self.save_to_disk()?;
		Ok(())
	}

	/// Get a view
	pub fn get_view(&self, name: &str) -> Option<&View> {
		self.views.get(name)
	}

	/// Check if a view exists
	pub fn view_exists(&self, name: &str) -> bool {
		self.views.contains_key(name)
	}

	/// List all view names
	pub fn list_views(&self) -> Vec<String> {
		self.views.keys().cloned().collect()
	}

	/// Drop a view
	pub fn drop_view(&mut self, name: &str) -> Result<()> {
		if self.views.remove(name).is_some() {
			self.mark_dirty();
			self.save_to_disk()?;
			Ok(())
		} else {
			Err(Error::NotFound(format!("View '{}' not found", name)))
		}
	}

	/// Create a trigger
	pub fn create_trigger(&mut self, stmt: crate::eplite::command::parser::CreateTriggerStatement) -> Result<()> {
		if self.triggers.contains_key(&stmt.name) {
			return Err(Error::Constraint(format!("Trigger '{}' already exists", stmt.name)));
		}
		
		// Check if the target table exists
		if !self.tables.contains_key(&stmt.table) {
			return Err(Error::NotFound(format!("Table '{}' not found", stmt.table)));
		}
		
		self.triggers.insert(stmt.name.clone(), stmt);
		Ok(())
	}

	/// Drop a trigger
	pub fn drop_trigger(&mut self, name: &str) -> Result<()> {
		if self.triggers.remove(name).is_some() {
			Ok(())
		} else {
			Err(Error::NotFound(format!("Trigger '{}' not found", name)))
		}
	}

	/// List all trigger names
	pub fn list_triggers(&self) -> Vec<String> {
		self.triggers.keys().cloned().collect()
	}

	/// Get triggers for a specific table and event
	pub fn get_triggers_for_table(&self, table: &str, event: &crate::eplite::command::parser::TriggerEvent, timing: &crate::eplite::command::parser::TriggerTiming) -> Vec<&crate::eplite::command::parser::CreateTriggerStatement> {
		self.triggers.values()
			.filter(|t| t.table == table && t.event == *event && t.timing == *timing)
			.collect()
	}

	/// Flush any pending changes to disk
	pub fn flush(&mut self) -> Result<()> {
		self.save_to_disk()
	}

	/// Begin a transaction
	pub fn begin_transaction(&mut self) -> Result<()> {
		if let Some(pager) = &mut self.pager {
			pager.begin_transaction()
		} else {
			Ok(())
		}
	}

	/// Commit a transaction
	pub fn commit_transaction(&mut self) -> Result<()> {
		// Save any dirty data first
		self.save_to_disk()?;
		
		if let Some(pager) = &mut self.pager {
			pager.commit_transaction()
		} else {
			Ok(())
		}
	}

	/// Rollback a transaction
	pub fn rollback_transaction(&mut self) -> Result<()> {
		if let Some(pager) = &mut self.pager {
			pager.rollback_transaction()?;
			// Reload from disk after rollback
			self.load_from_disk()?;
		}
		Ok(())
	}

	/// Perform a checkpoint (WAL mode only)
	#[cfg(feature = "std")]
	pub fn checkpoint(&mut self, mode: crate::eplite::persistence::wal::CheckpointMode) -> Result<()> {
		if let Some(pager) = &mut self.pager {
			pager.checkpoint(mode)
		} else {
			Ok(())
		}
	}

	/// Perform a simple CROSS JOIN between two tables (Cartesian product)
	pub fn cross_join(&self, table1_name: &str, table2_name: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
		let table1 = self.tables.get(table1_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table1_name)))?;
		let table2 = self.tables.get(table2_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table2_name)))?;

		// Get all rows from both tables
		let rows1 = table1.select_all();
		let rows2 = table2.select_all();

		// Build column names with table prefixes
		let mut column_names = Vec::new();
		for col in &table1.columns {
			column_names.push(format!("{}.{}", table1_name, col.name));
		}
		for col in &table2.columns {
			column_names.push(format!("{}.{}", table2_name, col.name));
		}

		// Cartesian product
		let mut result_rows = Vec::new();
		for row1 in &rows1 {
			for row2 in &rows2 {
				let mut combined_row = row1.clone();
				combined_row.extend(row2.clone());
				result_rows.push(combined_row);
			}
		}

		Ok((result_rows, column_names))
	}

	/// Perform INNER JOIN between two tables with ON condition
	pub fn inner_join(&self, table1_name: &str, table2_name: &str, on_condition: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
		let table1 = self.tables.get(table1_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table1_name)))?;
		let table2 = self.tables.get(table2_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table2_name)))?;

		// Parse ON condition (e.g., "table1.id = table2.user_id")
		let (left_col, right_col) = self.parse_join_condition(on_condition)?;

		// Find column indices
		let left_idx = self.find_column_in_table(table1, table1_name, &left_col)?;
		let right_idx = self.find_column_in_table(table2, table2_name, &right_col)?;

		// Get all rows from both tables
		let rows1 = table1.select_all();
		let rows2 = table2.select_all();

		// Build column names with table prefixes
		let mut column_names = Vec::new();
		for col in &table1.columns {
			column_names.push(format!("{}.{}", table1_name, col.name));
		}
		for col in &table2.columns {
			column_names.push(format!("{}.{}", table2_name, col.name));
		}

		// Inner join - only matching rows
		let mut result_rows = Vec::new();
		for row1 in &rows1 {
			for row2 in &rows2 {
				if left_idx < row1.len() && right_idx < row2.len() && row1[left_idx] == row2[right_idx] {
					let mut combined_row = row1.clone();
					combined_row.extend(row2.clone());
					result_rows.push(combined_row);
				}
			}
		}

		Ok((result_rows, column_names))
	}

	/// Perform LEFT JOIN between two tables with ON condition
	pub fn left_join(&self, table1_name: &str, table2_name: &str, on_condition: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
		let table1 = self.tables.get(table1_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table1_name)))?;
		let table2 = self.tables.get(table2_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table2_name)))?;

		// Parse ON condition (e.g., "table1.id = table2.user_id")
		let (left_col, right_col) = self.parse_join_condition(on_condition)?;

		// Find column indices
		let left_idx = self.find_column_in_table(table1, table1_name, &left_col)?;
		let right_idx = self.find_column_in_table(table2, table2_name, &right_col)?;

		// Get all rows from both tables
		let rows1 = table1.select_all();
		let rows2 = table2.select_all();

		// Build column names with table prefixes
		let mut column_names = Vec::new();
		for col in &table1.columns {
			column_names.push(format!("{}.{}", table1_name, col.name));
		}
		for col in &table2.columns {
			column_names.push(format!("{}.{}", table2_name, col.name));
		}

		// Left join - all rows from left table, matched rows from right table
		let mut result_rows = Vec::new();
		for row1 in &rows1 {
			let mut matched = false;
			for row2 in &rows2 {
				if left_idx < row1.len() && right_idx < row2.len() && row1[left_idx] == row2[right_idx] {
					let mut combined_row = row1.clone();
					combined_row.extend(row2.clone());
					result_rows.push(combined_row);
					matched = true;
				}
			}
			// If no match, add row with NULLs for right table
			if !matched {
				let mut combined_row = row1.clone();
				for _ in 0..table2.columns.len() {
					combined_row.push("NULL".to_string());
				}
				result_rows.push(combined_row);
			}
		}

		Ok((result_rows, column_names))
	}

	/// Perform RIGHT JOIN between two tables with ON condition
	pub fn right_join(&self, table1_name: &str, table2_name: &str, on_condition: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
		let table1 = self.tables.get(table1_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table1_name)))?;
		let table2 = self.tables.get(table2_name)
			.ok_or_else(|| Error::NotFound(format!("Table '{}' not found", table2_name)))?;

		// Parse ON condition (e.g., "table1.id = table2.user_id")
		let (left_col, right_col) = self.parse_join_condition(on_condition)?;

		// Find column indices
		let left_idx = self.find_column_in_table(table1, table1_name, &left_col)?;
		let right_idx = self.find_column_in_table(table2, table2_name, &right_col)?;

		// Get all rows from both tables
		let rows1 = table1.select_all();
		let rows2 = table2.select_all();

		// Build column names with table prefixes
		let mut column_names = Vec::new();
		for col in &table1.columns {
			column_names.push(format!("{}.{}", table1_name, col.name));
		}
		for col in &table2.columns {
			column_names.push(format!("{}.{}", table2_name, col.name));
		}

		// Right join - all rows from right table, matched rows from left table
		let mut result_rows = Vec::new();
		for row2 in &rows2 {
			let mut matched = false;
			for row1 in &rows1 {
				if left_idx < row1.len() && right_idx < row2.len() && row1[left_idx] == row2[right_idx] {
					let mut combined_row = row1.clone();
					combined_row.extend(row2.clone());
					result_rows.push(combined_row);
					matched = true;
				}
			}
			// If no match, add row with NULLs for left table
			if !matched {
				let mut combined_row = Vec::new();
				for _ in 0..table1.columns.len() {
					combined_row.push("NULL".to_string());
				}
				combined_row.extend(row2.clone());
				result_rows.push(combined_row);
			}
		}

		Ok((result_rows, column_names))
	}

	/// Parse JOIN ON condition like "table1.col1 = table2.col2"
	fn parse_join_condition(&self, condition: &str) -> Result<(String, String)> {
		let parts: Vec<&str> = condition.split('=').map(|s| s.trim()).collect();
		if parts.len() != 2 {
			return Err(Error::Syntax(format!("Invalid JOIN condition: {}", condition)));
		}
		Ok((parts[0].to_string(), parts[1].to_string()))
	}

	/// Find column index in table, supporting table.column notation
	fn find_column_in_table(&self, table: &Table, table_name: &str, column_ref: &str) -> Result<usize> {
		// Handle table.column or just column
		let column_ref = column_ref.trim();
		let column_name = if column_ref.contains('.') {
			let parts: Vec<&str> = column_ref.split('.').map(|s| s.trim()).collect();
			if parts.len() == 2 {
				// Verify table name matches
				if parts[0] != table_name {
					return Err(Error::Syntax(format!("Table name mismatch: {} vs {}", parts[0], table_name)));
				}
				parts[1]
			} else {
				return Err(Error::Syntax(format!("Invalid column reference: {}", column_ref)));
			}
		} else {
			column_ref
		};

		table.columns.iter()
			.position(|c| c.name == column_name)
			.ok_or_else(|| Error::NotFound(format!("Column '{}' not found in table '{}'", column_name, table_name)))
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

	#[test]
	fn test_left_join() {
		let mut manager = StorageManager::new();

		// Create users table
		let users_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![
				ColumnDefinition {
					name: "id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
				ColumnDefinition {
					name: "name".to_string(),
					data_type: ColumnType::Text,
					constraints: vec![],
				},
			],
		};
		manager.create_table(users_def).unwrap();

		// Create orders table
		let orders_def = CreateTableStatement {
			name: "orders".to_string(),
			columns: vec![
				ColumnDefinition {
					name: "order_id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
				ColumnDefinition {
					name: "user_id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
			],
		};
		manager.create_table(orders_def).unwrap();

		// Insert test data
		manager.get_table_mut("users").unwrap().insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		manager.get_table_mut("users").unwrap().insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		manager.get_table_mut("users").unwrap().insert(vec!["3".to_string(), "Charlie".to_string()]).unwrap();
		manager.get_table_mut("orders").unwrap().insert(vec!["101".to_string(), "1".to_string()]).unwrap();
		manager.get_table_mut("orders").unwrap().insert(vec!["102".to_string(), "2".to_string()]).unwrap();

		// Perform LEFT JOIN
		let (rows, columns) = manager.left_join("users", "orders", "users.id = orders.user_id").unwrap();

		// Should have 3 rows (all users, Charlie with NULL for orders)
		assert_eq!(rows.len(), 3);
		assert_eq!(columns.len(), 4); // 2 from users + 2 from orders

		// Check that Charlie (id=3) has NULLs for order data
		let charlie_row = rows.iter().find(|r| r[0] == "3").unwrap();
		assert_eq!(charlie_row[2], "NULL"); // order_id should be NULL
		assert_eq!(charlie_row[3], "NULL"); // user_id should be NULL
	}

	#[test]
	fn test_right_join() {
		let mut manager = StorageManager::new();

		// Create users table
		let users_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![
				ColumnDefinition {
					name: "id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
				ColumnDefinition {
					name: "name".to_string(),
					data_type: ColumnType::Text,
					constraints: vec![],
				},
			],
		};
		manager.create_table(users_def).unwrap();

		// Create orders table
		let orders_def = CreateTableStatement {
			name: "orders".to_string(),
			columns: vec![
				ColumnDefinition {
					name: "order_id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
				ColumnDefinition {
					name: "user_id".to_string(),
					data_type: ColumnType::Int32,
					constraints: vec![],
				},
			],
		};
		manager.create_table(orders_def).unwrap();

		// Insert test data - note order 103 has no matching user
		manager.get_table_mut("users").unwrap().insert(vec!["1".to_string(), "Alice".to_string()]).unwrap();
		manager.get_table_mut("users").unwrap().insert(vec!["2".to_string(), "Bob".to_string()]).unwrap();
		manager.get_table_mut("orders").unwrap().insert(vec!["101".to_string(), "1".to_string()]).unwrap();
		manager.get_table_mut("orders").unwrap().insert(vec!["102".to_string(), "2".to_string()]).unwrap();
		manager.get_table_mut("orders").unwrap().insert(vec!["103".to_string(), "99".to_string()]).unwrap(); // No matching user

		// Perform RIGHT JOIN
		let (rows, columns) = manager.right_join("users", "orders", "users.id = orders.user_id").unwrap();

		// Should have 3 rows (all orders, order 103 with NULL for user data)
		assert_eq!(rows.len(), 3);
		assert_eq!(columns.len(), 4); // 2 from users + 2 from orders

		// Check that order 103 has NULLs for user data
		let orphan_order = rows.iter().find(|r| r[2] == "103").unwrap();
		assert_eq!(orphan_order[0], "NULL"); // user id should be NULL
		assert_eq!(orphan_order[1], "NULL"); // user name should be NULL
	}

	#[test]
	fn test_create_trigger() {
		use crate::eplite::command::parser::{CreateTriggerStatement, TriggerTiming, TriggerEvent, TriggerAction, InsertStatement};
		
		let mut mgr = StorageManager::new();
		
		// Create a table first
		let table_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};
		mgr.create_table(table_def).unwrap();
		
		// Create a trigger
		let trigger = CreateTriggerStatement {
			name: "audit_trigger".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "users".to_string(),
			for_each_row: true,
			when_condition: None,
			actions: vec![TriggerAction::Insert(InsertStatement {
				table: "audit".to_string(),
				columns: vec![],
				values: vec!["test".to_string()],
			})],
		};
		
		let result = mgr.create_trigger(trigger);
		assert!(result.is_ok());
		
		// Verify trigger exists
		let triggers = mgr.list_triggers();
		assert_eq!(triggers.len(), 1);
		assert!(triggers.contains(&"audit_trigger".to_string()));
	}

	#[test]
	fn test_create_trigger_duplicate_name() {
		use crate::eplite::command::parser::{CreateTriggerStatement, TriggerTiming, TriggerEvent};
		
		let mut mgr = StorageManager::new();
		
		// Create a table first
		let table_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};
		mgr.create_table(table_def).unwrap();
		
		// Create first trigger
		let trigger1 = CreateTriggerStatement {
			name: "my_trigger".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "users".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		mgr.create_trigger(trigger1).unwrap();
		
		// Try to create duplicate trigger
		let trigger2 = CreateTriggerStatement {
			name: "my_trigger".to_string(),
			timing: TriggerTiming::After,
			event: TriggerEvent::Delete,
			table: "users".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		
		let result = mgr.create_trigger(trigger2);
		assert!(result.is_err());
	}

	#[test]
	fn test_create_trigger_table_not_found() {
		use crate::eplite::command::parser::{CreateTriggerStatement, TriggerTiming, TriggerEvent};
		
		let mut mgr = StorageManager::new();
		
		// Try to create trigger on non-existent table
		let trigger = CreateTriggerStatement {
			name: "my_trigger".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "nonexistent".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		
		let result = mgr.create_trigger(trigger);
		assert!(result.is_err());
	}

	#[test]
	fn test_drop_trigger() {
		use crate::eplite::command::parser::{CreateTriggerStatement, TriggerTiming, TriggerEvent};
		
		let mut mgr = StorageManager::new();
		
		// Create a table and trigger
		let table_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};
		mgr.create_table(table_def).unwrap();
		
		let trigger = CreateTriggerStatement {
			name: "audit_trigger".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "users".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		mgr.create_trigger(trigger).unwrap();
		
		// Drop the trigger
		let result = mgr.drop_trigger("audit_trigger");
		assert!(result.is_ok());
		
		// Verify trigger is gone
		let triggers = mgr.list_triggers();
		assert_eq!(triggers.len(), 0);
	}

	#[test]
	fn test_drop_trigger_not_found() {
		let mut mgr = StorageManager::new();
		
		let result = mgr.drop_trigger("nonexistent");
		assert!(result.is_err());
	}

	#[test]
	fn test_get_triggers_for_table() {
		use crate::eplite::command::parser::{CreateTriggerStatement, TriggerTiming, TriggerEvent};
		
		let mut mgr = StorageManager::new();
		
		// Create tables
		let table_def = CreateTableStatement {
			name: "users".to_string(),
			columns: vec![],
		};
		mgr.create_table(table_def).unwrap();
		
		let table_def2 = CreateTableStatement {
			name: "products".to_string(),
			columns: vec![],
		};
		mgr.create_table(table_def2).unwrap();
		
		// Create triggers
		let trigger1 = CreateTriggerStatement {
			name: "users_before_insert".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "users".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		mgr.create_trigger(trigger1).unwrap();
		
		let trigger2 = CreateTriggerStatement {
			name: "users_after_insert".to_string(),
			timing: TriggerTiming::After,
			event: TriggerEvent::Insert,
			table: "users".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		mgr.create_trigger(trigger2).unwrap();
		
		let trigger3 = CreateTriggerStatement {
			name: "products_before_insert".to_string(),
			timing: TriggerTiming::Before,
			event: TriggerEvent::Insert,
			table: "products".to_string(),
			for_each_row: false,
			when_condition: None,
			actions: vec![],
		};
		mgr.create_trigger(trigger3).unwrap();
		
		// Get triggers for users table, BEFORE, INSERT
		let triggers = mgr.get_triggers_for_table("users", &TriggerEvent::Insert, &TriggerTiming::Before);
		assert_eq!(triggers.len(), 1);
		assert_eq!(triggers[0].name, "users_before_insert");
		
		// Get triggers for users table, AFTER, INSERT
		let triggers = mgr.get_triggers_for_table("users", &TriggerEvent::Insert, &TriggerTiming::After);
		assert_eq!(triggers.len(), 1);
		assert_eq!(triggers[0].name, "users_after_insert");
		
		// Get triggers for products table
		let triggers = mgr.get_triggers_for_table("products", &TriggerEvent::Insert, &TriggerTiming::Before);
		assert_eq!(triggers.len(), 1);
		assert_eq!(triggers[0].name, "products_before_insert");
	}
}
