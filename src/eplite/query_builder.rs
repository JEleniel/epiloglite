/// Query builder for constructing SQL queries using a fluent interface

use crate::eplite::error::{Error, Result};

/// Trait for query builders
pub trait QueryBuilder {
	/// Build the SQL query string
	fn build(self) -> Result<String>;
}

/// Builder for SELECT queries
#[derive(Debug, Clone)]
pub struct SelectBuilder {
	columns: Vec<String>,
	from: Option<String>,
	where_clause: Option<String>,
	order_by: Vec<String>,
	limit: Option<usize>,
	offset: Option<usize>,
}

impl SelectBuilder {
	/// Create a new SELECT builder
	pub fn new() -> Self {
		SelectBuilder {
			columns: Vec::new(),
			from: None,
			where_clause: None,
			order_by: Vec::new(),
			limit: None,
			offset: None,
		}
	}

	/// Select all columns (*)
	pub fn select_all(mut self) -> Self {
		self.columns = vec!["*".to_string()];
		self
	}

	/// Select specific columns
	pub fn select<S: Into<String>>(mut self, columns: &[S]) -> Self
	where
		S: Clone,
	{
		self.columns = columns.iter().map(|c| c.clone().into()).collect();
		self
	}

	/// Add a column to select
	pub fn column<S: Into<String>>(mut self, column: S) -> Self {
		self.columns.push(column.into());
		self
	}

	/// Specify the table to select from
	pub fn from<S: Into<String>>(mut self, table: S) -> Self {
		self.from = Some(table.into());
		self
	}

	/// Add a WHERE clause
	pub fn where_clause<S: Into<String>>(mut self, condition: S) -> Self {
		self.where_clause = Some(condition.into());
		self
	}

	/// Add an ORDER BY clause
	pub fn order_by<S: Into<String>>(mut self, column: S) -> Self {
		self.order_by.push(column.into());
		self
	}

	/// Add a LIMIT clause
	pub fn limit(mut self, limit: usize) -> Self {
		self.limit = Some(limit);
		self
	}

	/// Add an OFFSET clause
	pub fn offset(mut self, offset: usize) -> Self {
		self.offset = Some(offset);
		self
	}

	/// Build the SQL query string
	pub fn build(self) -> Result<String> {
		if self.from.is_none() {
			return Err(Error::Syntax("FROM clause is required".to_string()));
		}

		let columns = if self.columns.is_empty() {
			"*".to_string()
		} else {
			self.columns.join(", ")
		};

		let mut sql = format!("SELECT {} FROM {}", columns, self.from.unwrap());

		if let Some(where_clause) = self.where_clause {
			sql.push_str(&format!(" WHERE {}", where_clause));
		}

		if !self.order_by.is_empty() {
			sql.push_str(&format!(" ORDER BY {}", self.order_by.join(", ")));
		}

		if let Some(limit) = self.limit {
			sql.push_str(&format!(" LIMIT {}", limit));
		}

		if let Some(offset) = self.offset {
			sql.push_str(&format!(" OFFSET {}", offset));
		}

		Ok(sql)
	}
}

impl QueryBuilder for SelectBuilder {
	fn build(self) -> Result<String> {
		SelectBuilder::build(self)
	}
}

impl Default for SelectBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for INSERT queries
#[derive(Debug, Clone)]
pub struct InsertBuilder {
	table: Option<String>,
	columns: Vec<String>,
	values: Vec<String>,
}

impl InsertBuilder {
	/// Create a new INSERT builder
	pub fn new() -> Self {
		InsertBuilder {
			table: None,
			columns: Vec::new(),
			values: Vec::new(),
		}
	}

	/// Specify the table to insert into
	pub fn into<S: Into<String>>(mut self, table: S) -> Self {
		self.table = Some(table.into());
		self
	}

	/// Specify columns (optional)
	pub fn columns<S: Into<String>>(mut self, columns: &[S]) -> Self
	where
		S: Clone,
	{
		self.columns = columns.iter().map(|c| c.clone().into()).collect();
		self
	}

	/// Add a value
	pub fn value<S: Into<String>>(mut self, value: S) -> Self {
		self.values.push(value.into());
		self
	}

	/// Add multiple values
	pub fn values<S: Into<String>>(mut self, values: &[S]) -> Self
	where
		S: Clone,
	{
		self.values = values.iter().map(|v| v.clone().into()).collect();
		self
	}

	/// Build the SQL query string
	pub fn build(self) -> Result<String> {
		if self.table.is_none() {
			return Err(Error::Syntax("Table name is required".to_string()));
		}

		if self.values.is_empty() {
			return Err(Error::Syntax("VALUES are required".to_string()));
		}

		let mut sql = format!("INSERT INTO {}", self.table.unwrap());

		if !self.columns.is_empty() {
			sql.push_str(&format!(" ({})", self.columns.join(", ")));
		}

		sql.push_str(&format!(" VALUES ({})", self.values.join(", ")));

		Ok(sql)
	}
}

impl Default for InsertBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for UPDATE queries
#[derive(Debug, Clone)]
pub struct UpdateBuilder {
	table: Option<String>,
	set_clauses: Vec<(String, String)>,
	where_clause: Option<String>,
}

impl UpdateBuilder {
	/// Create a new UPDATE builder
	pub fn new() -> Self {
		UpdateBuilder {
			table: None,
			set_clauses: Vec::new(),
			where_clause: None,
		}
	}

	/// Specify the table to update
	pub fn table<S: Into<String>>(mut self, table: S) -> Self {
		self.table = Some(table.into());
		self
	}

	/// Add a SET clause
	pub fn set<S: Into<String>>(mut self, column: S, value: S) -> Self {
		self.set_clauses.push((column.into(), value.into()));
		self
	}

	/// Add a WHERE clause
	pub fn where_clause<S: Into<String>>(mut self, condition: S) -> Self {
		self.where_clause = Some(condition.into());
		self
	}

	/// Build the SQL query string
	pub fn build(self) -> Result<String> {
		if self.table.is_none() {
			return Err(Error::Syntax("Table name is required".to_string()));
		}

		if self.set_clauses.is_empty() {
			return Err(Error::Syntax("SET clause is required".to_string()));
		}

		let set_parts: Vec<String> = self
			.set_clauses
			.iter()
			.map(|(col, val)| format!("{} = {}", col, val))
			.collect();

		let mut sql = format!("UPDATE {} SET {}", self.table.unwrap(), set_parts.join(", "));

		if let Some(where_clause) = self.where_clause {
			sql.push_str(&format!(" WHERE {}", where_clause));
		}

		Ok(sql)
	}
}

impl Default for UpdateBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for DELETE queries
#[derive(Debug, Clone)]
pub struct DeleteBuilder {
	table: Option<String>,
	where_clause: Option<String>,
}

impl DeleteBuilder {
	/// Create a new DELETE builder
	pub fn new() -> Self {
		DeleteBuilder {
			table: None,
			where_clause: None,
		}
	}

	/// Specify the table to delete from
	pub fn from<S: Into<String>>(mut self, table: S) -> Self {
		self.table = Some(table.into());
		self
	}

	/// Add a WHERE clause
	pub fn where_clause<S: Into<String>>(mut self, condition: S) -> Self {
		self.where_clause = Some(condition.into());
		self
	}

	/// Build the SQL query string
	pub fn build(self) -> Result<String> {
		if self.table.is_none() {
			return Err(Error::Syntax("Table name is required".to_string()));
		}

		let mut sql = format!("DELETE FROM {}", self.table.unwrap());

		if let Some(where_clause) = self.where_clause {
			sql.push_str(&format!(" WHERE {}", where_clause));
		}

		Ok(sql)
	}
}

impl Default for DeleteBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for CREATE TABLE queries
#[derive(Debug, Clone)]
pub struct CreateTableBuilder {
	name: Option<String>,
	columns: Vec<(String, String, Vec<String>)>, // (name, type, constraints)
}

impl CreateTableBuilder {
	/// Create a new CREATE TABLE builder
	pub fn new() -> Self {
		CreateTableBuilder {
			name: None,
			columns: Vec::new(),
		}
	}

	/// Specify the table name
	pub fn table<S: Into<String>>(mut self, name: S) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Add a column
	pub fn column<S: Into<String>>(
		mut self,
		name: S,
		data_type: S,
		constraints: &[S],
	) -> Self
	where
		S: Clone,
	{
		let constraints_vec: Vec<String> = constraints.iter().map(|c| c.clone().into()).collect();
		self.columns
			.push((name.into(), data_type.into(), constraints_vec));
		self
	}

	/// Add a simple column without constraints
	pub fn simple_column<S: Into<String>>(mut self, name: S, data_type: S) -> Self {
		self.columns
			.push((name.into(), data_type.into(), Vec::new()));
		self
	}

	/// Build the SQL query string
	pub fn build(self) -> Result<String> {
		if self.name.is_none() {
			return Err(Error::Syntax("Table name is required".to_string()));
		}

		if self.columns.is_empty() {
			return Err(Error::Syntax(
				"At least one column is required".to_string(),
			));
		}

		let column_defs: Vec<String> = self
			.columns
			.iter()
			.map(|(name, data_type, constraints)| {
				let mut def = format!("{} {}", name, data_type);
				if !constraints.is_empty() {
					def.push_str(" ");
					def.push_str(&constraints.join(" "));
				}
				def
			})
			.collect();

		let sql = format!(
			"CREATE TABLE {} ({})",
			self.name.unwrap(),
			column_defs.join(", ")
		);

		Ok(sql)
	}
}

impl Default for CreateTableBuilder {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_select_builder() {
		let sql = SelectBuilder::new()
			.select_all()
			.from("users")
			.build()
			.unwrap();
		assert_eq!(sql, "SELECT * FROM users");
	}

	#[test]
	fn test_select_with_where() {
		let sql = SelectBuilder::new()
			.column("name")
			.column("age")
			.from("users")
			.where_clause("age > 18")
			.build()
			.unwrap();
		assert_eq!(sql, "SELECT name, age FROM users WHERE age > 18");
	}

	#[test]
	fn test_select_with_order_limit() {
		let sql = SelectBuilder::new()
			.select_all()
			.from("users")
			.order_by("name")
			.limit(10)
			.build()
			.unwrap();
		assert_eq!(sql, "SELECT * FROM users ORDER BY name LIMIT 10");
	}

	#[test]
	fn test_insert_builder() {
		let sql = InsertBuilder::new()
			.into("users")
			.values(&["1", "'John'", "25"])
			.build()
			.unwrap();
		assert_eq!(sql, "INSERT INTO users VALUES (1, 'John', 25)");
	}

	#[test]
	fn test_insert_with_columns() {
		let sql = InsertBuilder::new()
			.into("users")
			.columns(&["id", "name"])
			.values(&["1", "'John'"])
			.build()
			.unwrap();
		assert_eq!(sql, "INSERT INTO users (id, name) VALUES (1, 'John')");
	}

	#[test]
	fn test_update_builder() {
		let sql = UpdateBuilder::new()
			.table("users")
			.set("name", "'Jane'")
			.set("age", "26")
			.where_clause("id = 1")
			.build()
			.unwrap();
		assert_eq!(sql, "UPDATE users SET name = 'Jane', age = 26 WHERE id = 1");
	}

	#[test]
	fn test_delete_builder() {
		let sql = DeleteBuilder::new()
			.from("users")
			.where_clause("id = 1")
			.build()
			.unwrap();
		assert_eq!(sql, "DELETE FROM users WHERE id = 1");
	}

	#[test]
	fn test_create_table_builder() {
		let sql = CreateTableBuilder::new()
			.table("users")
			.column("id", "INTEGER", &["PRIMARY KEY"])
			.simple_column("name", "TEXT")
			.build()
			.unwrap();
		assert_eq!(sql, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
	}
}

impl QueryBuilder for InsertBuilder {
fn build(self) -> Result<String> {
InsertBuilder::build(self)
}
}

impl QueryBuilder for UpdateBuilder {
fn build(self) -> Result<String> {
UpdateBuilder::build(self)
}
}

impl QueryBuilder for DeleteBuilder {
fn build(self) -> Result<String> {
DeleteBuilder::build(self)
}
}

impl QueryBuilder for CreateTableBuilder {
fn build(self) -> Result<String> {
CreateTableBuilder::build(self)
}
}
