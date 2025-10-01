/// SQL parser - assigns meaning to tokens based on context

use crate::eplite::error::{Error, Result};

/// Parse tree node types
#[derive(Debug, Clone)]
pub enum Statement {
	Select(SelectStatement),
	Insert(InsertStatement),
	Update(UpdateStatement),
	Delete(DeleteStatement),
	CreateTable(CreateTableStatement),
}

#[derive(Debug, Clone)]
pub struct SelectStatement {
	pub columns: Vec<String>,
	pub from: String,
	pub where_clause: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InsertStatement {
	pub table: String,
	pub columns: Vec<String>,
	pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateStatement {
	pub table: String,
	pub set_clauses: Vec<(String, String)>,
	pub where_clause: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteStatement {
	pub table: String,
	pub where_clause: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTableStatement {
	pub name: String,
	pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
	pub name: String,
	pub data_type: String,
	pub constraints: Vec<String>,
}

/// SQL parser
#[derive(Debug)]
pub struct Parser {}

impl Parser {
	pub fn new() -> Self {
		Parser {}
	}

	/// Parse SQL statement into a parse tree
	pub fn parse(&self, _sql: &str) -> Result<Statement> {
		// TODO: Implement actual parsing
		Err(Error::NotSupported(
			"SQL parsing not yet implemented".to_string(),
		))
	}
}

impl Default for Parser {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parser_creation() {
		let parser = Parser::new();
		assert!(format!("{:?}", parser).contains("Parser"));
	}
}
