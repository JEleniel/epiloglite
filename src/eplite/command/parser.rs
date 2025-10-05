/// SQL parser - assigns meaning to tokens based on context

use crate::eplite::command::tokenizer::{Token, Tokenizer};
use crate::eplite::error::{Error, Result};
use crate::eplite::types::column::ColumnType;
use logos::Logos;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::{format, string::{String, ToString}, vec, vec::Vec};

/// Parse tree node types
#[derive(Debug, Clone)]
pub enum Statement {
	Select(SelectStatement),
	Insert(InsertStatement),
	Update(UpdateStatement),
	Delete(DeleteStatement),
	CreateTable(CreateTableStatement),
	CreateView(CreateViewStatement),
	DropView(DropViewStatement),
	BeginTransaction,
	Commit,
	Rollback,
	Savepoint(String),
	Release(String),
	RollbackToSavepoint(String),
}

/// Aggregate function type
#[derive(Debug, Clone, PartialEq)]
pub enum AggregateFunction {
	Count,
	Sum,
	Avg,
	Min,
	Max,
}

/// Column selection - either a regular column or an aggregate
#[derive(Debug, Clone)]
pub enum ColumnSelection {
	Column(String),
	Aggregate {
		function: AggregateFunction,
		column: String,
	},
	CountStar, // COUNT(*)
}

/// Join type
#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
	Inner,
	Left,
	Right,
	Cross,
}

/// Join clause
#[derive(Debug, Clone)]
pub struct JoinClause {
	pub join_type: JoinType,
	pub table: String,
	pub on_condition: Option<String>, // e.g., "table1.id = table2.id"
}

#[derive(Debug, Clone)]
pub struct SelectStatement {
	pub columns: Vec<ColumnSelection>,
	pub from: String,
	pub joins: Vec<JoinClause>,
	pub where_clause: Option<String>,
	pub group_by: Option<Vec<String>>,
	pub order_by: Option<Vec<String>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableStatement {
	pub name: String,
	pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateViewStatement {
	pub name: String,
	pub query: String,
}

#[derive(Debug, Clone)]
pub struct DropViewStatement {
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
	pub name: String,
	pub data_type: ColumnType,
	pub constraints: Vec<String>,
}

/// SQL parser
#[derive(Debug)]
pub struct Parser {
	tokens: Vec<Token>,
	position: usize,
	source: String,
}

impl Parser {
	pub fn new() -> Self {
		Parser {
			tokens: Vec::new(),
			position: 0,
			source: String::new(),
		}
	}

	/// Parse SQL statement into a parse tree
	pub fn parse(&mut self, sql: &str) -> Result<Statement> {
		// Store the source for extracting actual text
		self.source = sql.to_string();

		// Tokenize the SQL
		let tokenizer = Tokenizer::new(sql.to_string());
		self.tokens = tokenizer.tokenize();
		self.position = 0;

		if self.tokens.is_empty() {
			return Err(Error::Syntax("Empty statement".to_string()));
		}

		// Parse based on first token
		let stmt = match self.current_token() {
			Some(Token::Select) => self.parse_select()?,
			Some(Token::Insert) => self.parse_insert()?,
			Some(Token::Update) => self.parse_update()?,
			Some(Token::Delete) => self.parse_delete()?,
			Some(Token::Create) => self.parse_create()?,
			Some(Token::Drop) => self.parse_drop()?,
			Some(Token::Begin) => {
				self.advance();
				Statement::BeginTransaction
			}
			Some(Token::Commit) => {
				self.advance();
				Statement::Commit
			}
			Some(Token::Rollback) => {
				self.advance();
				self.parse_rollback()?
			}
			Some(Token::Savepoint) => {
				self.advance();
				self.parse_savepoint()?
			}
			Some(Token::Release) => {
				self.advance();
				self.parse_release()?
			}
			_ => {
				return Err(Error::Syntax(format!(
					"Unexpected token: {:?}",
					self.current_token()
				)))
			}
		};

		// Check for extra tokens after statement
		if self.position < self.tokens.len() {
			return Err(Error::Syntax(format!(
				"Unexpected tokens after statement: {:?}",
				self.current_token()
			)));
		}

		Ok(stmt)
	}

	fn current_token(&self) -> Option<&Token> {
		self.tokens.get(self.position)
	}

	fn advance(&mut self) {
		self.position += 1;
	}

	fn expect(&mut self, expected: Token) -> Result<()> {
		match self.current_token() {
			Some(token) if token == &expected => {
				self.advance();
				Ok(())
			}
			Some(token) => Err(Error::Syntax(format!(
				"Expected {:?}, found {:?}",
				expected, token
			))),
			None => Err(Error::Syntax("Unexpected end of statement".to_string())),
		}
	}

	fn parse_identifier(&mut self) -> Result<String> {
		match self.current_token() {
			Some(Token::Identifier) => {
				// Extract the actual text by re-lexing from the source
				let mut lex = Token::lexer(&self.source);
				let mut idx = 0;
				while let Some(token) = lex.next() {
					if idx == self.position {
						if let Ok(Token::Identifier) = token {
							let text = lex.slice().to_string();
							self.advance();
							return Ok(text);
						}
					}
					idx += 1;
				}
				
				// Fallback
				self.advance();
				Ok(format!("identifier_{}", self.position))
			}
			_ => Err(Error::Syntax(format!(
				"Expected identifier, found {:?}",
				self.current_token()
			))),
		}
	}

	/// Parse WHERE clause - returns the raw text of the condition
	fn parse_where_clause(&mut self) -> Result<String> {
		// Collect all tokens until we hit a keyword that ends WHERE clause
		let mut parts = Vec::new();
		
		// Re-lex to get actual text
		let mut lex = Token::lexer(&self.source);
		let mut token_texts = Vec::new();
		while let Some(token) = lex.next() {
			if token.is_ok() {
				token_texts.push(lex.slice().to_string());
			}
		}
		
		loop {
			match self.current_token() {
				None => break,
				Some(Token::Set) |
				Some(Token::Order) |
				Some(Token::Group) |
				Some(Token::Limit) |
				Some(Token::Offset) => break,
				Some(_) => {
					if self.position < token_texts.len() {
						parts.push(token_texts[self.position].clone());
					}
					self.advance();
				}
			}
		}
		
		if parts.is_empty() {
			Err(Error::Syntax("Empty WHERE clause".to_string()))
		} else {
			Ok(parts.join(" "))
		}
	}

	/// Parse a value token (number or string literal)
	fn parse_value(&mut self) -> Result<String> {
		// Re-lex to get actual text
		let mut lex = Token::lexer(&self.source);
		let mut token_texts = Vec::new();
		while let Some(token) = lex.next() {
			if token.is_ok() {
				token_texts.push(lex.slice().to_string());
			}
		}
		
		if self.position < token_texts.len() {
			let val = token_texts[self.position].clone();
			self.advance();
			Ok(val)
		} else {
			self.advance();
			Ok("NULL".to_string())
		}
	}

	fn parse_select(&mut self) -> Result<Statement> {
		self.expect(Token::Select)?;

		let mut columns = Vec::new();
		
		// Parse columns (including aggregates)
		if matches!(self.current_token(), Some(Token::Star)) {
			columns.push(ColumnSelection::Column("*".to_string()));
			self.advance();
		} else {
			loop {
				// Check for aggregate functions
				let col = match self.current_token() {
					Some(Token::Count) => {
						self.advance();
						self.expect(Token::LeftParen)?;
						if matches!(self.current_token(), Some(Token::Star)) {
							self.advance();
							self.expect(Token::RightParen)?;
							ColumnSelection::CountStar
						} else {
							let col_name = self.parse_identifier()?;
							self.expect(Token::RightParen)?;
							ColumnSelection::Aggregate {
								function: AggregateFunction::Count,
								column: col_name,
							}
						}
					}
					Some(Token::Sum) => {
						self.advance();
						self.expect(Token::LeftParen)?;
						let col_name = self.parse_identifier()?;
						self.expect(Token::RightParen)?;
						ColumnSelection::Aggregate {
							function: AggregateFunction::Sum,
							column: col_name,
						}
					}
					Some(Token::Avg) => {
						self.advance();
						self.expect(Token::LeftParen)?;
						let col_name = self.parse_identifier()?;
						self.expect(Token::RightParen)?;
						ColumnSelection::Aggregate {
							function: AggregateFunction::Avg,
							column: col_name,
						}
					}
					Some(Token::Min) => {
						self.advance();
						self.expect(Token::LeftParen)?;
						let col_name = self.parse_identifier()?;
						self.expect(Token::RightParen)?;
						ColumnSelection::Aggregate {
							function: AggregateFunction::Min,
							column: col_name,
						}
					}
					Some(Token::Max) => {
						self.advance();
						self.expect(Token::LeftParen)?;
						let col_name = self.parse_identifier()?;
						self.expect(Token::RightParen)?;
						ColumnSelection::Aggregate {
							function: AggregateFunction::Max,
							column: col_name,
						}
					}
					_ => ColumnSelection::Column(self.parse_identifier()?),
				};
				
				columns.push(col);
				
				if !matches!(self.current_token(), Some(Token::Comma)) {
					break;
				}
				self.advance();
			}
		}

		self.expect(Token::From)?;
		let from = self.parse_identifier()?;

		// Parse JOIN clauses
		let mut joins = Vec::new();
		while matches!(
			self.current_token(),
			Some(Token::Join) | Some(Token::Inner) | Some(Token::Left) | Some(Token::Right) | Some(Token::Cross)
		) {
			joins.push(self.parse_join_clause()?);
		}

		let where_clause = if matches!(self.current_token(), Some(Token::Where)) {
			self.advance();
			Some(self.parse_where_clause()?)
		} else {
			None
		};

		// Parse GROUP BY
		let group_by = if matches!(self.current_token(), Some(Token::Group)) {
			self.advance();
			self.expect(Token::By)?;
			let mut cols = Vec::new();
			loop {
				cols.push(self.parse_identifier()?);
				if !matches!(self.current_token(), Some(Token::Comma)) {
					break;
				}
				self.advance();
			}
			Some(cols)
		} else {
			None
		};

		// Parse ORDER BY
		let order_by = if matches!(self.current_token(), Some(Token::Order)) {
			self.advance();
			self.expect(Token::By)?;
			let mut cols = Vec::new();
			loop {
				cols.push(self.parse_identifier()?);
				// Skip ASC/DESC keywords if present
				if matches!(self.current_token(), Some(Token::Asc) | Some(Token::Desc)) {
					self.advance();
				}
				if !matches!(self.current_token(), Some(Token::Comma)) {
					break;
				}
				self.advance();
			}
			Some(cols)
		} else {
			None
		};

		Ok(Statement::Select(SelectStatement {
			columns,
			from,
			joins,
			where_clause,
			group_by,
			order_by,
		}))
	}

	fn parse_insert(&mut self) -> Result<Statement> {
		self.expect(Token::Insert)?;
		self.expect(Token::Into)?;
		
		let table = self.parse_identifier()?;
		
		// Optional column list
		let columns = if matches!(self.current_token(), Some(Token::LeftParen)) {
			self.advance();
			let mut cols = Vec::new();
			loop {
				cols.push(self.parse_identifier()?);
				if !matches!(self.current_token(), Some(Token::Comma)) {
					break;
				}
				self.advance();
			}
			self.expect(Token::RightParen)?;
			cols
		} else {
			Vec::new()
		};

		self.expect(Token::Values)?;
		self.expect(Token::LeftParen)?;
		
		let mut values = Vec::new();
		loop {
			// Parse actual value
			values.push(self.parse_value()?);
			
			if !matches!(self.current_token(), Some(Token::Comma)) {
				break;
			}
			self.advance();
		}
		
		self.expect(Token::RightParen)?;

		Ok(Statement::Insert(InsertStatement {
			table,
			columns,
			values,
		}))
	}

	fn parse_update(&mut self) -> Result<Statement> {
		self.expect(Token::Update)?;
		let table = self.parse_identifier()?;
		self.expect(Token::Set)?;

		let mut set_clauses = Vec::new();
		loop {
			let col = self.parse_identifier()?;
			self.expect(Token::Equals)?;
			let val = self.parse_value()?;
			set_clauses.push((col, val));
			
			if !matches!(self.current_token(), Some(Token::Comma)) {
				break;
			}
			self.advance();
		}

		let where_clause = if matches!(self.current_token(), Some(Token::Where)) {
			self.advance();
			Some(self.parse_where_clause()?)
		} else {
			None
		};

		Ok(Statement::Update(UpdateStatement {
			table,
			set_clauses,
			where_clause,
		}))
	}

	fn parse_delete(&mut self) -> Result<Statement> {
		self.expect(Token::Delete)?;
		self.expect(Token::From)?;
		let table = self.parse_identifier()?;

		let where_clause = if matches!(self.current_token(), Some(Token::Where)) {
			self.advance();
			Some(self.parse_where_clause()?)
		} else {
			None
		};

		Ok(Statement::Delete(DeleteStatement {
			table,
			where_clause,
		}))
	}

	fn parse_create(&mut self) -> Result<Statement> {
		self.expect(Token::Create)?;
		
		// Check if it's a TABLE or VIEW
		match self.current_token() {
			Some(Token::Table) => self.parse_create_table(),
			Some(Token::View) => self.parse_create_view(),
			_ => Err(Error::Syntax("Expected TABLE or VIEW after CREATE".to_string())),
		}
	}

	fn parse_create_table(&mut self) -> Result<Statement> {
		self.expect(Token::Table)?;
		
		let name = self.parse_identifier()?;
		self.expect(Token::LeftParen)?;

		let mut columns = Vec::new();
		loop {
			let col_name = self.parse_identifier()?;
			
			// Parse data type
			let data_type = match self.current_token() {
				Some(Token::Integer) => {
					self.advance();
					ColumnType::Int32
				}
				Some(Token::Text) => {
					self.advance();
					ColumnType::Text
				}
				Some(Token::Real) => {
					self.advance();
					ColumnType::Float32
				}
				Some(Token::Blob) => {
					self.advance();
					ColumnType::Blob
				}
				Some(Token::Boolean) => {
					self.advance();
					ColumnType::Boolean
				}
				_ => return Err(Error::Syntax("Expected data type".to_string())),
			};

			// Parse constraints
			let mut constraints = Vec::new();
			while matches!(
				self.current_token(),
				Some(Token::Primary) | Some(Token::Not) | Some(Token::Unique) | Some(Token::Default)
			) {
				if matches!(self.current_token(), Some(Token::Primary)) {
					self.advance();
					if matches!(self.current_token(), Some(Token::Key)) {
						self.advance();
						constraints.push("PRIMARY KEY".to_string());
					}
				} else if matches!(self.current_token(), Some(Token::Not)) {
					self.advance();
					if matches!(self.current_token(), Some(Token::Null)) {
						self.advance();
						constraints.push("NOT NULL".to_string());
					}
				} else if matches!(self.current_token(), Some(Token::Unique)) {
					self.advance();
					constraints.push("UNIQUE".to_string());
				} else if matches!(self.current_token(), Some(Token::Default)) {
					self.advance();
					// Skip the default value
					self.advance();
					constraints.push("DEFAULT".to_string());
				}
			}

			columns.push(ColumnDefinition {
				name: col_name,
				data_type,
				constraints,
			});

			if !matches!(self.current_token(), Some(Token::Comma)) {
				break;
			}
			self.advance();
		}

		self.expect(Token::RightParen)?;

		Ok(Statement::CreateTable(CreateTableStatement {
			name,
			columns,
		}))
	}

	fn parse_create_view(&mut self) -> Result<Statement> {
		self.expect(Token::View)?;
		
		let name = self.parse_identifier()?;
		self.expect(Token::As)?;
		
		// Extract the query from the source SQL
		// Find "AS" in the source and extract everything after it (excluding semicolon)
		let query = if let Some(as_pos) = self.source.to_uppercase().rfind(" AS ") {
			let after_as = &self.source[as_pos + 4..];
			// Trim semicolon if present
			after_as.trim_end_matches(';').trim().to_string()
		} else {
			return Err(Error::Syntax("Could not extract view query".to_string()));
		};
		
		// Advance past the query tokens
		while self.position < self.tokens.len() {
			if matches!(self.current_token(), Some(Token::Semicolon)) {
				break;
			}
			self.advance();
		}
		
		Ok(Statement::CreateView(CreateViewStatement {
			name,
			query,
		}))
	}

	fn parse_drop(&mut self) -> Result<Statement> {
		self.expect(Token::Drop)?;
		
		match self.current_token() {
			Some(Token::View) => {
				self.advance();
				let name = self.parse_identifier()?;
				Ok(Statement::DropView(DropViewStatement { name }))
			}
			_ => Err(Error::Syntax("Expected VIEW after DROP".to_string())),
		}
	}

	fn parse_rollback(&mut self) -> Result<Statement> {
		// Check if this is "ROLLBACK TO SAVEPOINT name" or "ROLLBACK TO name"
		if matches!(self.current_token(), Some(Token::To)) {
			self.advance(); // consume TO
			
			// Optional SAVEPOINT keyword
			if matches!(self.current_token(), Some(Token::Savepoint)) {
				self.advance();
			}
			
			// Get savepoint name
			let name = self.parse_identifier()?;
			Ok(Statement::RollbackToSavepoint(name))
		} else if matches!(self.current_token(), Some(Token::Transaction)) {
			// ROLLBACK TRANSACTION
			self.advance();
			Ok(Statement::Rollback)
		} else {
			// Just ROLLBACK
			Ok(Statement::Rollback)
		}
	}

	fn parse_savepoint(&mut self) -> Result<Statement> {
		// SAVEPOINT name
		let name = self.parse_identifier()?;
		Ok(Statement::Savepoint(name))
	}

	fn parse_release(&mut self) -> Result<Statement> {
		// RELEASE [SAVEPOINT] name
		
		// Optional SAVEPOINT keyword
		if matches!(self.current_token(), Some(Token::Savepoint)) {
			self.advance();
		}
		
		// Get savepoint name
		let name = self.parse_identifier()?;
		Ok(Statement::Release(name))
	}

	fn parse_join_clause(&mut self) -> Result<JoinClause> {
		// Determine join type
		let join_type = match self.current_token() {
			Some(Token::Cross) => {
				self.advance();
				self.expect(Token::Join)?;
				JoinType::Cross
			}
			Some(Token::Inner) => {
				self.advance();
				self.expect(Token::Join)?;
				JoinType::Inner
			}
			Some(Token::Left) => {
				self.advance();
				// Optional OUTER keyword
				if matches!(self.current_token(), Some(Token::Outer)) {
					self.advance();
				}
				self.expect(Token::Join)?;
				JoinType::Left
			}
			Some(Token::Right) => {
				self.advance();
				// Optional OUTER keyword
				if matches!(self.current_token(), Some(Token::Outer)) {
					self.advance();
				}
				self.expect(Token::Join)?;
				JoinType::Right
			}
			Some(Token::Join) => {
				// Just JOIN defaults to INNER JOIN
				self.advance();
				JoinType::Inner
			}
			_ => {
				return Err(Error::Syntax("Expected JOIN keyword".to_string()));
			}
		};

		// Get table name
		let table = self.parse_identifier()?;

		// Parse ON condition (except for CROSS JOIN)
		let on_condition = if join_type != JoinType::Cross {
			self.expect(Token::On)?;
			Some(self.parse_where_clause()?)
		} else {
			None
		};

		Ok(JoinClause {
			join_type,
			table,
			on_condition,
		})
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

	#[test]
	fn test_parse_select() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.columns.len(), 1);
				assert!(matches!(stmt.columns[0], ColumnSelection::Column(ref s) if s == "*"));
			}
			_ => panic!("Expected Select statement"),
		}
	}

	#[test]
	fn test_parse_insert() {
		let mut parser = Parser::new();
		let result = parser.parse("INSERT INTO users VALUES (1, 'John')");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Insert(stmt) => {
				assert!(!stmt.table.is_empty());
			}
			_ => panic!("Expected Insert statement"),
		}
	}

	#[test]
	fn test_parse_create_table() {
		let mut parser = Parser::new();
		let result = parser.parse("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::CreateTable(stmt) => {
				assert_eq!(stmt.columns.len(), 2);
				assert_eq!(stmt.columns[0].data_type, ColumnType::Int32);
				assert!(stmt.columns[0].constraints.contains(&"PRIMARY KEY".to_string()));
			}
			_ => panic!("Expected CreateTable statement"),
		}
	}

	#[test]
	fn test_parse_update() {
		let mut parser = Parser::new();
		let result = parser.parse("UPDATE users SET name = 'Jane' WHERE id = 1");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Update(stmt) => {
				assert_eq!(stmt.set_clauses.len(), 1);
			}
			_ => panic!("Expected Update statement"),
		}
	}

	#[test]
	fn test_parse_delete() {
		let mut parser = Parser::new();
		let result = parser.parse("DELETE FROM users WHERE id = 1");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Delete(stmt) => {
				assert!(!stmt.table.is_empty());
				assert!(stmt.where_clause.is_some());
			}
			_ => panic!("Expected Delete statement"),
		}
	}

	#[test]
	fn test_parse_transactions() {
		let mut parser = Parser::new();
		
		let result = parser.parse("BEGIN");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), Statement::BeginTransaction));

		let result = parser.parse("COMMIT");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), Statement::Commit));

		let result = parser.parse("ROLLBACK");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), Statement::Rollback));
	}

	#[test]
	fn test_parse_savepoint() {
		let mut parser = Parser::new();
		
		// Test SAVEPOINT name
		let result = parser.parse("SAVEPOINT sp1");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Savepoint(name) => {
				assert_eq!(name, "sp1");
			}
			_ => panic!("Expected Savepoint statement"),
		}
	}

	#[test]
	fn test_parse_release() {
		let mut parser = Parser::new();
		
		// Test RELEASE name
		let result = parser.parse("RELEASE sp1");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Release(name) => {
				assert_eq!(name, "sp1");
			}
			_ => panic!("Expected Release statement"),
		}

		// Test RELEASE SAVEPOINT name
		let result = parser.parse("RELEASE SAVEPOINT sp2");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Release(name) => {
				assert_eq!(name, "sp2");
			}
			_ => panic!("Expected Release statement"),
		}
	}

	#[test]
	fn test_parse_rollback_to_savepoint() {
		let mut parser = Parser::new();
		
		// Test ROLLBACK TO name
		let result = parser.parse("ROLLBACK TO sp1");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::RollbackToSavepoint(name) => {
				assert_eq!(name, "sp1");
			}
			_ => panic!("Expected RollbackToSavepoint statement"),
		}

		// Test ROLLBACK TO SAVEPOINT name
		let result = parser.parse("ROLLBACK TO SAVEPOINT sp2");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::RollbackToSavepoint(name) => {
				assert_eq!(name, "sp2");
			}
			_ => panic!("Expected RollbackToSavepoint statement"),
		}

		// Test ROLLBACK TRANSACTION (should still parse as Rollback)
		let result = parser.parse("ROLLBACK TRANSACTION");
		assert!(result.is_ok());
		assert!(matches!(result.unwrap(), Statement::Rollback));
	}

	#[test]
	fn test_parse_invalid_statement() {
		let mut parser = Parser::new();
		let result = parser.parse("INVALID STATEMENT");
		assert!(result.is_err());
	}

	#[test]
	fn test_parse_inner_join() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.from, "users");
				assert_eq!(stmt.joins.len(), 1);
				assert_eq!(stmt.joins[0].join_type, JoinType::Inner);
				assert_eq!(stmt.joins[0].table, "orders");
				assert!(stmt.joins[0].on_condition.is_some());
			}
			_ => panic!("Expected Select statement"),
		}
	}

	#[test]
	fn test_parse_left_join() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users LEFT JOIN orders ON users.id = orders.user_id");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.joins.len(), 1);
				assert_eq!(stmt.joins[0].join_type, JoinType::Left);
			}
			_ => panic!("Expected Select statement"),
		}
	}

	#[test]
	fn test_parse_right_join() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users RIGHT JOIN orders ON users.id = orders.user_id");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.joins.len(), 1);
				assert_eq!(stmt.joins[0].join_type, JoinType::Right);
			}
			_ => panic!("Expected Select statement"),
		}
	}

	#[test]
	fn test_parse_cross_join() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users CROSS JOIN orders");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.joins.len(), 1);
				assert_eq!(stmt.joins[0].join_type, JoinType::Cross);
				assert!(stmt.joins[0].on_condition.is_none());
			}
			_ => panic!("Expected Select statement"),
		}
	}

	#[test]
	fn test_parse_left_outer_join() {
		let mut parser = Parser::new();
		let result = parser.parse("SELECT * FROM users LEFT OUTER JOIN orders ON users.id = orders.user_id");
		assert!(result.is_ok());
		match result.unwrap() {
			Statement::Select(stmt) => {
				assert_eq!(stmt.joins.len(), 1);
				assert_eq!(stmt.joins[0].join_type, JoinType::Left);
			}
			_ => panic!("Expected Select statement"),
		}
	}
}
