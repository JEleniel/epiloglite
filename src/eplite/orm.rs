/// Lightweight ORM (Object-Relational Mapping) for EpilogLite
///
/// Provides a type-safe way to work with database entities

use crate::eplite::command::processor::ExecutionResult;
use crate::eplite::database::Database;
use crate::eplite::error::{Error, Result};
use crate::eplite::types::column::ColumnType;
use std::collections::HashMap;

/// Trait for entities that can be persisted to the database
pub trait Entity: Sized {
	/// Get the table name for this entity
	fn table_name() -> &'static str;
	
	/// Get the primary key column name
	fn primary_key() -> &'static str {
		"id"
	}
	
	/// Convert entity to field map (column_name -> value)
	fn to_fields(&self) -> HashMap<String, String>;
	
	/// Create entity from field map
	fn from_fields(fields: &HashMap<String, String>) -> Result<Self>;
	
	/// Get column definitions for CREATE TABLE
	fn column_definitions() -> Vec<ColumnDefinition>;
}

/// Column definition for ORM
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
	pub name: String,
	pub column_type: ColumnType,
	pub constraints: Vec<String>,
}

impl ColumnDefinition {
	pub fn new(name: impl Into<String>, column_type: ColumnType) -> Self {
		ColumnDefinition {
			name: name.into(),
			column_type,
			constraints: Vec::new(),
		}
	}
	
	pub fn primary_key(mut self) -> Self {
		self.constraints.push("PRIMARY KEY".to_string());
		self
	}
	
	pub fn not_null(mut self) -> Self {
		self.constraints.push("NOT NULL".to_string());
		self
	}
	
	pub fn unique(mut self) -> Self {
		self.constraints.push("UNIQUE".to_string());
		self
	}
	
	pub fn default_value(mut self, value: impl Into<String>) -> Self {
		self.constraints.push(format!("DEFAULT {}", value.into()));
		self
	}
}

/// Repository pattern for working with entities
pub struct Repository<'a, T: Entity> {
	db: &'a mut Database,
	_phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Entity> Repository<'a, T> {
	/// Create a new repository
	pub fn new(db: &'a mut Database) -> Self {
		Repository {
			db,
			_phantom: std::marker::PhantomData,
		}
	}
	
	/// Get database reference
	fn db(&mut self) -> &mut Database {
		self.db
	}
	
	/// Create table for entity if it doesn't exist
	pub fn create_table(&mut self) -> Result<()> {
		let table_name = T::table_name();
		let definitions = T::column_definitions();
		
		// Build CREATE TABLE statement
		let mut sql = format!("CREATE TABLE {} (", table_name);
		for (i, def) in definitions.iter().enumerate() {
			if i > 0 {
				sql.push_str(", ");
			}
			sql.push_str(&def.name);
			sql.push(' ');
			
			// Map ColumnType to SQL type name
			let type_name = match def.column_type {
				ColumnType::Int32 | ColumnType::Int64 => "INTEGER",
				ColumnType::Text => "TEXT",
				ColumnType::Blob => "BLOB",
				ColumnType::Float64 => "REAL",
				_ => "TEXT",
			};
			sql.push_str(type_name);
			
			// Add constraints
			for constraint in &def.constraints {
				sql.push(' ');
				sql.push_str(constraint);
			}
		}
		sql.push(')');
		
		self.db().execute(&sql)?;
		Ok(())
	}
	
	/// Insert an entity
	pub fn insert(&mut self, entity: &T) -> Result<()> {
		let table_name = T::table_name();
		let fields = entity.to_fields();
		
		// Build INSERT statement
		let columns: Vec<&String> = fields.keys().collect();
		let values: Vec<&String> = fields.values().collect();
		
		let mut sql = format!("INSERT INTO {} (", table_name);
		for (i, col) in columns.iter().enumerate() {
			if i > 0 {
				sql.push_str(", ");
			}
			sql.push_str(col);
		}
		sql.push_str(") VALUES (");
		for (i, val) in values.iter().enumerate() {
			if i > 0 {
				sql.push_str(", ");
			}
			sql.push_str(val);
		}
		sql.push(')');
		
		self.db().execute(&sql)?;
		Ok(())
	}
	
	/// Find entity by primary key
	pub fn find(&mut self, id: impl ToString) -> Result<Option<T>> {
		let table_name = T::table_name();
		let pk = T::primary_key();
		let sql = format!("SELECT * FROM {} WHERE {} = {}", table_name, pk, id.to_string());
		
		let result = self.db().execute(&sql)?;
		
		if let ExecutionResult::Select { rows, columns } = result {
			if rows.is_empty() {
				return Ok(None);
			}
			
			// Convert first row to entity
			let mut fields = HashMap::new();
			for (i, col) in columns.iter().enumerate() {
				if let Some(value) = rows[0].get(i) {
					fields.insert(col.clone(), value.clone());
				}
			}
			
			Ok(Some(T::from_fields(&fields)?))
		} else {
			Err(Error::Internal("Expected SELECT result".to_string()))
		}
	}
	
	/// Find all entities
	pub fn find_all(&mut self) -> Result<Vec<T>> {
		let table_name = T::table_name();
		let sql = format!("SELECT * FROM {}", table_name);
		
		let result = self.db().execute(&sql)?;
		
		if let ExecutionResult::Select { rows, columns } = result {
			let mut entities = Vec::new();
			
			for row in rows {
				let mut fields = HashMap::new();
				for (i, col) in columns.iter().enumerate() {
					if let Some(value) = row.get(i) {
						fields.insert(col.clone(), value.clone());
					}
				}
				entities.push(T::from_fields(&fields)?);
			}
			
			Ok(entities)
		} else {
			Err(Error::Internal("Expected SELECT result".to_string()))
		}
	}
	
	/// Update an entity (by primary key)
	pub fn update(&mut self, entity: &T) -> Result<()> {
		let table_name = T::table_name();
		let pk = T::primary_key();
		let fields = entity.to_fields();
		
		// Extract PK value
		let pk_value = fields.get(pk)
			.ok_or_else(|| Error::Internal("Primary key not found in entity".to_string()))?;
		
		// Build UPDATE statement
		let mut sql = format!("UPDATE {} SET ", table_name);
		let mut first = true;
		for (col, val) in &fields {
			if col == pk {
				continue; // Skip PK in SET clause
			}
			if !first {
				sql.push_str(", ");
			}
			sql.push_str(&format!("{} = {}", col, val));
			first = false;
		}
		sql.push_str(&format!(" WHERE {} = {}", pk, pk_value));
		
		self.db().execute(&sql)?;
		Ok(())
	}
	
	/// Delete an entity by primary key
	pub fn delete(&mut self, id: impl ToString) -> Result<()> {
		let table_name = T::table_name();
		let pk = T::primary_key();
		let sql = format!("DELETE FROM {} WHERE {} = {}", table_name, pk, id.to_string());
		
		self.db().execute(&sql)?;
		Ok(())
	}
}

/// Example entity for testing
#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
struct User {
	id: i32,
	name: String,
	email: String,
	age: i32,
}

#[cfg(test)]
impl Entity for User {
	fn table_name() -> &'static str {
		"users"
	}
	
	fn to_fields(&self) -> HashMap<String, String> {
		let mut fields = HashMap::new();
		fields.insert("id".to_string(), self.id.to_string());
		fields.insert("name".to_string(), format!("'{}'", self.name));
		fields.insert("email".to_string(), format!("'{}'", self.email));
		fields.insert("age".to_string(), self.age.to_string());
		fields
	}
	
	fn from_fields(fields: &HashMap<String, String>) -> Result<Self> {
		Ok(User {
			id: fields.get("id")
				.ok_or_else(|| Error::Internal("Missing id".to_string()))?
				.parse()
				.map_err(|_| Error::TypeMismatch("Invalid id".to_string()))?,
			name: fields.get("name")
				.ok_or_else(|| Error::Internal("Missing name".to_string()))?
				.trim_matches('\'')
				.to_string(),
			email: fields.get("email")
				.ok_or_else(|| Error::Internal("Missing email".to_string()))?
				.trim_matches('\'')
				.to_string(),
			age: fields.get("age")
				.ok_or_else(|| Error::Internal("Missing age".to_string()))?
				.parse()
				.map_err(|_| Error::TypeMismatch("Invalid age".to_string()))?,
		})
	}
	
	fn column_definitions() -> Vec<ColumnDefinition> {
		vec![
			ColumnDefinition::new("id", ColumnType::Int32).primary_key(),
			ColumnDefinition::new("name", ColumnType::Text).not_null(),
			ColumnDefinition::new("email", ColumnType::Text).unique(),
			ColumnDefinition::new("age", ColumnType::Int32),
		]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_column_definition_builder() {
		let col = ColumnDefinition::new("id", ColumnType::Int32)
			.primary_key()
			.not_null();
		
		assert_eq!(col.name, "id");
		assert_eq!(col.constraints, vec!["PRIMARY KEY", "NOT NULL"]);
	}
	
	#[test]
	fn test_entity_to_fields() {
		let user = User {
			id: 1,
			name: "Alice".to_string(),
			email: "alice@example.com".to_string(),
			age: 30,
		};
		
		let fields = user.to_fields();
		assert_eq!(fields.get("id"), Some(&"1".to_string()));
		assert_eq!(fields.get("name"), Some(&"'Alice'".to_string()));
		assert_eq!(fields.get("age"), Some(&"30".to_string()));
	}
	
	#[test]
	fn test_entity_from_fields() -> Result<()> {
		let mut fields = HashMap::new();
		fields.insert("id".to_string(), "1".to_string());
		fields.insert("name".to_string(), "'Alice'".to_string());
		fields.insert("email".to_string(), "'alice@example.com'".to_string());
		fields.insert("age".to_string(), "30".to_string());
		
		let user = User::from_fields(&fields)?;
		assert_eq!(user.id, 1);
		assert_eq!(user.name, "Alice");
		assert_eq!(user.age, 30);
		
		Ok(())
	}
	
	#[test]
	fn test_repository_basic() -> Result<()> {
		let mut db = Database::open(":memory:")?;
		let mut repo = Repository::<User>::new(&mut db);
		
		// Create table
		repo.create_table()?;
		
		// Insert entity
		let user = User {
			id: 1,
			name: "Alice".to_string(),
			email: "alice@example.com".to_string(),
			age: 30,
		};
		repo.insert(&user)?;
		
		// For now, just verify insert worked
		// Full find() functionality needs WHERE clause implementation
		// which is in progress
		
		Ok(())
	}
}