/// Stored procedures management module

use crate::eplite::command::parser::{CreateProcedureStatement, ProcedureBodyStatement};
use crate::eplite::error::{Error, Result};
use crate::eplite::types::ValueType;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "std")]
use std::collections::BTreeMap;

/// Stored procedure registry
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcedureRegistry {
	procedures: BTreeMap<String, StoredProcedure>,
}

/// A stored procedure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredProcedure {
	pub name: String,
	pub definition: CreateProcedureStatement,
}

/// Context for executing a procedure
#[derive(Debug)]
pub struct ProcedureContext {
	pub variables: BTreeMap<String, ValueType>,
	pub parameters: BTreeMap<String, ValueType>,
}

impl ProcedureRegistry {
	pub fn new() -> Self {
		ProcedureRegistry {
			procedures: BTreeMap::new(),
		}
	}

	/// Register a new stored procedure
	pub fn create_procedure(&mut self, definition: CreateProcedureStatement) -> Result<()> {
		let name = definition.name.clone();
		
		if self.procedures.contains_key(&name) {
			return Err(Error::AlreadyExists(format!(
				"Procedure '{}' already exists",
				name
			)));
		}
		
		self.procedures.insert(
			name.clone(),
			StoredProcedure {
				name,
				definition,
			},
		);
		
		Ok(())
	}

	/// Drop a stored procedure
	pub fn drop_procedure(&mut self, name: &str) -> Result<()> {
		if self.procedures.remove(name).is_none() {
			return Err(Error::NotFound(format!("Procedure '{}' not found", name)));
		}
		Ok(())
	}

	/// Get a stored procedure by name
	pub fn get_procedure(&self, name: &str) -> Option<&StoredProcedure> {
		self.procedures.get(name)
	}

	/// List all procedure names
	pub fn list_procedures(&self) -> Vec<String> {
		self.procedures.keys().cloned().collect()
	}
}

impl ProcedureContext {
	pub fn new() -> Self {
		ProcedureContext {
			variables: BTreeMap::new(),
			parameters: BTreeMap::new(),
		}
	}

	/// Set a variable value
	pub fn set_variable(&mut self, name: String, value: ValueType) {
		self.variables.insert(name, value);
	}

	/// Get a variable value
	pub fn get_variable(&self, name: &str) -> Option<&ValueType> {
		self.variables.get(name)
	}

	/// Set a parameter value
	pub fn set_parameter(&mut self, name: String, value: ValueType) {
		self.parameters.insert(name, value);
	}

	/// Get a parameter value
	pub fn get_parameter(&self, name: &str) -> Option<&ValueType> {
		self.parameters.get(name)
	}
}

/// Execute a procedure body statement
pub fn execute_statement(
	_stmt: &ProcedureBodyStatement,
	_context: &mut ProcedureContext,
) -> Result<Option<ValueType>> {
	// This will be implemented in the processor
	// For now, return a placeholder
	Ok(None)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::eplite::command::parser::{ProcedureParameter, ParameterMode};
	use crate::eplite::types::column::ColumnType;

	#[test]
	fn test_procedure_registry_creation() {
		let registry = ProcedureRegistry::new();
		assert_eq!(registry.list_procedures().len(), 0);
	}

	#[test]
	fn test_create_procedure() {
		let mut registry = ProcedureRegistry::new();
		
		let definition = CreateProcedureStatement {
			name: "test_proc".to_string(),
			parameters: vec![ProcedureParameter {
				name: "x".to_string(),
				data_type: ColumnType::Int32,
				mode: ParameterMode::In,
			}],
			body: vec![],
		};
		
		assert!(registry.create_procedure(definition).is_ok());
		assert_eq!(registry.list_procedures().len(), 1);
	}

	#[test]
	fn test_duplicate_procedure() {
		let mut registry = ProcedureRegistry::new();
		
		let definition = CreateProcedureStatement {
			name: "test_proc".to_string(),
			parameters: vec![],
			body: vec![],
		};
		
		assert!(registry.create_procedure(definition.clone()).is_ok());
		assert!(registry.create_procedure(definition).is_err());
	}

	#[test]
	fn test_drop_procedure() {
		let mut registry = ProcedureRegistry::new();
		
		let definition = CreateProcedureStatement {
			name: "test_proc".to_string(),
			parameters: vec![],
			body: vec![],
		};
		
		registry.create_procedure(definition).unwrap();
		assert!(registry.drop_procedure("test_proc").is_ok());
		assert_eq!(registry.list_procedures().len(), 0);
	}

	#[test]
	fn test_drop_nonexistent_procedure() {
		let mut registry = ProcedureRegistry::new();
		assert!(registry.drop_procedure("nonexistent").is_err());
	}

	#[test]
	fn test_procedure_context() {
		let mut context = ProcedureContext::new();
		context.set_variable("x".to_string(), ValueType::I32(42));
		
		assert_eq!(context.get_variable("x"), Some(&ValueType::I32(42)));
		assert_eq!(context.get_variable("y"), None);
	}
}
