/// SQL parser - assigns meaning to tokens based on context

use crate::eplite::command::tokenizer::{Token, Tokenizer};
use crate::eplite::error::{Error, Result};
use crate::eplite::types::column::ColumnType;
use logos::Logos;
use serde::{Deserialize, Serialize};

/// Parse tree node types
#[derive(Debug, Clone)]
pub enum Statement {
	Select(SelectStatement),
	Insert(InsertStatement),
	Update(UpdateStatement),
	Delete(DeleteStatement),
	CreateTable(CreateTableStatement),
	BeginTransaction,
	Commit,
	Rollback,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableStatement {
	pub name: String,
	pub columns: Vec<ColumnDefinition>,
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
				Statement::Rollback
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
		
		// Parse columns
		if matches!(self.current_token(), Some(Token::Star)) {
			columns.push("*".to_string());
			self.advance();
		} else {
			loop {
				columns.push(self.parse_identifier()?);
				if !matches!(self.current_token(), Some(Token::Comma)) {
					break;
				}
				self.advance();
			}
		}

		self.expect(Token::From)?;
		let from = self.parse_identifier()?;

		let where_clause = if matches!(self.current_token(), Some(Token::Where)) {
			self.advance();
			Some(self.parse_where_clause()?)
		} else {
			None
		};

		Ok(Statement::Select(SelectStatement {
			columns,
			from,
			where_clause,
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
				assert_eq!(stmt.columns[0], "*");
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
	fn test_parse_invalid_statement() {
		let mut parser = Parser::new();
		let result = parser.parse("INVALID STATEMENT");
		assert!(result.is_err());
	}
}
