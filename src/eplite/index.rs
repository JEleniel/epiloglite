/// Indexing system for EpilogLite
///
/// Provides B-tree based indexes for faster lookups

use crate::eplite::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Index type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndexType {
	/// Primary key index (unique, non-null)
	Primary,
	/// Unique index (unique values only)
	Unique,
	/// Regular index (non-unique)
	Regular,
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
	/// Index name
	pub name: String,
	
	/// Table name this index belongs to
	pub table_name: String,
	
	/// Column name(s) this index covers
	pub columns: Vec<String>,
	
	/// Index type
	pub index_type: IndexType,
	
	/// Internal index data: maps indexed value to row IDs
	data: BTreeMap<String, Vec<usize>>,
}

impl Index {
	/// Create a new index
	pub fn new(name: String, table_name: String, columns: Vec<String>, index_type: IndexType) -> Self {
		Index {
			name,
			table_name,
			columns,
			index_type,
			data: BTreeMap::new(),
		}
	}
	
	/// Insert a value into the index
	pub fn insert(&mut self, value: String, row_id: usize) -> Result<()> {
		// For unique indexes, check if value already exists
		if self.index_type == IndexType::Unique || self.index_type == IndexType::Primary {
			if self.data.contains_key(&value) {
				return Err(Error::Constraint(format!(
					"Unique constraint violation: value '{}' already exists in index '{}'",
					value, self.name
				)));
			}
		}
		
		// Insert the value
		self.data.entry(value).or_insert_with(Vec::new).push(row_id);
		
		Ok(())
	}
	
	/// Remove a value from the index
	pub fn remove(&mut self, value: &str, row_id: usize) -> Result<()> {
		if let Some(row_ids) = self.data.get_mut(value) {
			row_ids.retain(|&id| id != row_id);
			if row_ids.is_empty() {
				self.data.remove(value);
			}
		}
		Ok(())
	}
	
	/// Look up row IDs by value
	pub fn lookup(&self, value: &str) -> Vec<usize> {
		self.data.get(value).cloned().unwrap_or_default()
	}
	
	/// Range query: find all values between start and end (inclusive)
	pub fn range(&self, start: &str, end: &str) -> Vec<(String, Vec<usize>)> {
		self.data
			.range(start.to_string()..=end.to_string())
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect()
	}
	
	/// Get all entries (for iteration)
	pub fn all(&self) -> Vec<(String, Vec<usize>)> {
		self.data
			.iter()
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect()
	}
	
	/// Update the index when a row is updated
	pub fn update(&mut self, old_value: &str, new_value: String, row_id: usize) -> Result<()> {
		self.remove(old_value, row_id)?;
		self.insert(new_value, row_id)?;
		Ok(())
	}
	
	/// Check if index contains a value
	pub fn contains(&self, value: &str) -> bool {
		self.data.contains_key(value)
	}
	
	/// Get size of index (number of unique values)
	pub fn size(&self) -> usize {
		self.data.len()
	}
	
	/// Clear all data from index
	pub fn clear(&mut self) {
		self.data.clear();
	}
}

/// Index manager for a database
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexManager {
	/// All indexes in the database
	indexes: Vec<Index>,
}

impl IndexManager {
	/// Create a new index manager
	pub fn new() -> Self {
		IndexManager {
			indexes: Vec::new(),
		}
	}
	
	/// Create a new index
	pub fn create_index(
		&mut self,
		name: String,
		table_name: String,
		columns: Vec<String>,
		index_type: IndexType,
	) -> Result<()> {
		// Check if index with same name already exists
		if self.indexes.iter().any(|idx| idx.name == name) {
			return Err(Error::Constraint(format!("Index '{}' already exists", name)));
		}
		
		let index = Index::new(name, table_name, columns, index_type);
		self.indexes.push(index);
		Ok(())
	}
	
	/// Drop an index
	pub fn drop_index(&mut self, name: &str) -> Result<()> {
		let initial_len = self.indexes.len();
		self.indexes.retain(|idx| idx.name != name);
		
		if self.indexes.len() == initial_len {
			Err(Error::NotFound(format!("Index '{}' not found", name)))
		} else {
			Ok(())
		}
	}
	
	/// Get an index by name
	pub fn get_index(&self, name: &str) -> Option<&Index> {
		self.indexes.iter().find(|idx| idx.name == name)
	}
	
	/// Get a mutable index by name
	pub fn get_index_mut(&mut self, name: &str) -> Option<&mut Index> {
		self.indexes.iter_mut().find(|idx| idx.name == name)
	}
	
	/// Get all indexes for a table
	pub fn get_table_indexes(&self, table_name: &str) -> Vec<&Index> {
		self.indexes
			.iter()
			.filter(|idx| idx.table_name == table_name)
			.collect()
	}
	
	/// List all index names
	pub fn list_indexes(&self) -> Vec<String> {
		self.indexes.iter().map(|idx| idx.name.clone()).collect()
	}
}

impl Default for IndexManager {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_create_index() {
		let index = Index::new(
			"idx_users_email".to_string(),
			"users".to_string(),
			vec!["email".to_string()],
			IndexType::Unique,
		);
		
		assert_eq!(index.name, "idx_users_email");
		assert_eq!(index.table_name, "users");
		assert_eq!(index.columns, vec!["email"]);
		assert_eq!(index.index_type, IndexType::Unique);
		assert_eq!(index.size(), 0);
	}
	
	#[test]
	fn test_index_insert_lookup() -> Result<()> {
		let mut index = Index::new(
			"idx_test".to_string(),
			"test".to_string(),
			vec!["col".to_string()],
			IndexType::Regular,
		);
		
		index.insert("alice".to_string(), 0)?;
		index.insert("bob".to_string(), 1)?;
		index.insert("alice".to_string(), 2)?; // Duplicate OK for regular index
		
		assert_eq!(index.lookup("alice"), vec![0, 2]);
		assert_eq!(index.lookup("bob"), vec![1]);
		assert_eq!(index.lookup("charlie"), Vec::<usize>::new());
		
		Ok(())
	}
	
	#[test]
	fn test_unique_index_constraint() {
		let mut index = Index::new(
			"idx_unique".to_string(),
			"test".to_string(),
			vec!["col".to_string()],
			IndexType::Unique,
		);
		
		assert!(index.insert("alice".to_string(), 0).is_ok());
		assert!(index.insert("alice".to_string(), 1).is_err()); // Should fail
	}
	
	#[test]
	fn test_index_remove() -> Result<()> {
		let mut index = Index::new(
			"idx_test".to_string(),
			"test".to_string(),
			vec!["col".to_string()],
			IndexType::Regular,
		);
		
		index.insert("alice".to_string(), 0)?;
		index.insert("alice".to_string(), 1)?;
		
		index.remove("alice", 0)?;
		assert_eq!(index.lookup("alice"), vec![1]);
		
		index.remove("alice", 1)?;
		assert_eq!(index.lookup("alice"), Vec::<usize>::new());
		
		Ok(())
	}
	
	#[test]
	fn test_index_range() -> Result<()> {
		let mut index = Index::new(
			"idx_test".to_string(),
			"test".to_string(),
			vec!["col".to_string()],
			IndexType::Regular,
		);
		
		index.insert("alice".to_string(), 0)?;
		index.insert("bob".to_string(), 1)?;
		index.insert("charlie".to_string(), 2)?;
		index.insert("david".to_string(), 3)?;
		
		let range_results = index.range("bob", "charlie");
		assert_eq!(range_results.len(), 2);
		assert_eq!(range_results[0].0, "bob");
		assert_eq!(range_results[1].0, "charlie");
		
		Ok(())
	}
	
	#[test]
	fn test_index_manager() -> Result<()> {
		let mut mgr = IndexManager::new();
		
		mgr.create_index(
			"idx1".to_string(),
			"users".to_string(),
			vec!["email".to_string()],
			IndexType::Unique,
		)?;
		
		mgr.create_index(
			"idx2".to_string(),
			"users".to_string(),
			vec!["name".to_string()],
			IndexType::Regular,
		)?;
		
		assert_eq!(mgr.list_indexes().len(), 2);
		assert!(mgr.get_index("idx1").is_some());
		assert!(mgr.get_index("idx3").is_none());
		
		mgr.drop_index("idx1")?;
		assert_eq!(mgr.list_indexes().len(), 1);
		
		Ok(())
	}
}