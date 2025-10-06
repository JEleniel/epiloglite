/// SQL tokenizer - breaks SQL text into tokens

use logos::Logos;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// SQL token types
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	// Keywords - Data Manipulation
	#[token("SELECT", ignore(ascii_case))]
	Select,
	#[token("FROM", ignore(ascii_case))]
	From,
	#[token("WHERE", ignore(ascii_case))]
	Where,
	#[token("INSERT", ignore(ascii_case))]
	Insert,
	#[token("INTO", ignore(ascii_case))]
	Into,
	#[token("VALUES", ignore(ascii_case))]
	Values,
	#[token("UPDATE", ignore(ascii_case))]
	Update,
	#[token("SET", ignore(ascii_case))]
	Set,
	#[token("DELETE", ignore(ascii_case))]
	Delete,
	
	// Keywords - Data Definition
	#[token("CREATE", ignore(ascii_case))]
	Create,
	#[token("DROP", ignore(ascii_case))]
	Drop,
	#[token("ALTER", ignore(ascii_case))]
	Alter,
	#[token("TABLE", ignore(ascii_case))]
	Table,
	#[token("INDEX", ignore(ascii_case))]
	Index,
	#[token("VIEW", ignore(ascii_case))]
	View,
	#[token("DATABASE", ignore(ascii_case))]
	Database,
	#[token("GRAPH", ignore(ascii_case))]
	Graph,
	
	// Keywords - Graph Operations
	#[token("NODE", ignore(ascii_case))]
	Node,
	#[token("EDGE", ignore(ascii_case))]
	Edge,
	#[token("ADD", ignore(ascii_case))]
	Add,
	#[token("MATCH", ignore(ascii_case))]
	Match,
	#[token("PATH", ignore(ascii_case))]
	Path,
	#[token("TRAVERSE", ignore(ascii_case))]
	Traverse,
	#[token("LABEL", ignore(ascii_case))]
	Label,
	#[token("WEIGHT", ignore(ascii_case))]
	Weight,
	#[token("PROPERTIES", ignore(ascii_case))]
	Properties,
	
	// Keywords - Constraints
	#[token("PRIMARY", ignore(ascii_case))]
	Primary,
	#[token("KEY", ignore(ascii_case))]
	Key,
	#[token("FOREIGN", ignore(ascii_case))]
	Foreign,
	#[token("REFERENCES", ignore(ascii_case))]
	References,
	#[token("UNIQUE", ignore(ascii_case))]
	Unique,
	#[token("NOT", ignore(ascii_case))]
	Not,
	#[token("NULL", ignore(ascii_case))]
	Null,
	#[token("DEFAULT", ignore(ascii_case))]
	Default,
	#[token("CHECK", ignore(ascii_case))]
	Check,
	
	// Keywords - Joins
	#[token("JOIN", ignore(ascii_case))]
	Join,
	#[token("INNER", ignore(ascii_case))]
	Inner,
	#[token("LEFT", ignore(ascii_case))]
	Left,
	#[token("RIGHT", ignore(ascii_case))]
	Right,
	#[token("OUTER", ignore(ascii_case))]
	Outer,
	#[token("CROSS", ignore(ascii_case))]
	Cross,
	#[token("ON", ignore(ascii_case))]
	On,
	#[token("TO", ignore(ascii_case))]
	To,
	#[token("IN", ignore(ascii_case))]
	In,
	#[token("USING", ignore(ascii_case))]
	Using,
	
	// Keywords - Sorting and Grouping
	#[token("ORDER", ignore(ascii_case))]
	Order,
	#[token("BY", ignore(ascii_case))]
	By,
	#[token("GROUP", ignore(ascii_case))]
	Group,
	#[token("HAVING", ignore(ascii_case))]
	Having,
	#[token("ASC", ignore(ascii_case))]
	Asc,
	#[token("DESC", ignore(ascii_case))]
	Desc,
	
	// Keywords - Logic
	#[token("AND", ignore(ascii_case))]
	And,
	#[token("OR", ignore(ascii_case))]
	Or,
	#[token("BETWEEN", ignore(ascii_case))]
	Between,
	#[token("LIKE", ignore(ascii_case))]
	Like,
	#[token("IS", ignore(ascii_case))]
	Is,
	#[token("EXISTS", ignore(ascii_case))]
	Exists,
	
	// Keywords - Aggregates
	#[token("COUNT", ignore(ascii_case))]
	Count,
	#[token("SUM", ignore(ascii_case))]
	Sum,
	#[token("AVG", ignore(ascii_case))]
	Avg,
	#[token("MIN", ignore(ascii_case))]
	Min,
	#[token("MAX", ignore(ascii_case))]
	Max,
	
	// Keywords - Transactions
	#[token("BEGIN", ignore(ascii_case))]
	Begin,
	#[token("COMMIT", ignore(ascii_case))]
	Commit,
	#[token("ROLLBACK", ignore(ascii_case))]
	Rollback,
	#[token("TRANSACTION", ignore(ascii_case))]
	Transaction,
	#[token("SAVEPOINT", ignore(ascii_case))]
	Savepoint,
	#[token("RELEASE", ignore(ascii_case))]
	Release,
	
	// Keywords - Other
	#[token("AS", ignore(ascii_case))]
	As,
	#[token("DISTINCT", ignore(ascii_case))]
	Distinct,
	#[token("LIMIT", ignore(ascii_case))]
	Limit,
	#[token("OFFSET", ignore(ascii_case))]
	Offset,
	#[token("CASE", ignore(ascii_case))]
	Case,
	#[token("WHEN", ignore(ascii_case))]
	When,
	#[token("THEN", ignore(ascii_case))]
	Then,
	#[token("ELSE", ignore(ascii_case))]
	Else,
	#[token("END", ignore(ascii_case))]
	End,
	
	// Data Types
	#[token("INTEGER", ignore(ascii_case))]
	Integer,
	#[token("TEXT", ignore(ascii_case))]
	Text,
	#[token("REAL", ignore(ascii_case))]
	Real,
	#[token("BLOB", ignore(ascii_case))]
	Blob,
	#[token("BOOLEAN", ignore(ascii_case))]
	Boolean,
	
	// Operators
	#[token("=")]
	Equals,
	#[token("!=")]
	NotEquals,
	#[token("<>")]
	NotEquals2,
	#[token("<")]
	LessThan,
	#[token(">")]
	GreaterThan,
	#[token("<=")]
	LessThanOrEqual,
	#[token(">=")]
	GreaterThanOrEqual,
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Star,
	#[token("/")]
	Slash,
	#[token("%")]
	Percent,
	#[token("||")]
	Concat,
	
	// Symbols
	#[token("(")]
	LeftParen,
	#[token(")")]
	RightParen,
	#[token(",")]
	Comma,
	#[token(";")]
	Semicolon,
	#[token(".")]
	Dot,

	// Identifiers and literals
	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
	Identifier,
	#[regex(r#"'[^']*'"#)]
	StringLiteral,
	#[regex(r#""[^"]*""#)]
	QuotedIdentifier,
	#[regex(r"[0-9]+\.[0-9]+")]
	FloatLiteral,
	#[regex(r"[0-9]+")]
	IntegerLiteral,
	#[regex(r"--[^\n]*", logos::skip)]
	Comment,
}

/// Tokenizer for SQL statements
pub struct Tokenizer {
	source: String,
}

impl Tokenizer {
	pub fn new(source: String) -> Self {
		Tokenizer { source }
	}

	pub fn tokenize(&self) -> Vec<Token> {
		let mut tokens = Vec::new();
		let lex = Token::lexer(&self.source);

		for token in lex.flatten() {
			tokens.push(token);
		}

		tokens
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize_simple_select() {
		let tokenizer = Tokenizer::new("SELECT * FROM users".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens.len(), 4);
		assert_eq!(tokens[0], Token::Select);
		assert_eq!(tokens[1], Token::Star);
		assert_eq!(tokens[2], Token::From);
		assert_eq!(tokens[3], Token::Identifier);
	}

	#[test]
	fn test_tokenize_case_insensitive() {
		let tokenizer = Tokenizer::new("select from where".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Select);
		assert_eq!(tokens[1], Token::From);
		assert_eq!(tokens[2], Token::Where);
	}

	#[test]
	fn test_tokenize_insert() {
		let tokenizer = Tokenizer::new("INSERT INTO users VALUES (1, 'John')".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Insert);
		assert_eq!(tokens[1], Token::Into);
		assert_eq!(tokens[2], Token::Identifier);
		assert_eq!(tokens[3], Token::Values);
	}

	#[test]
	fn test_tokenize_operators() {
		let tokenizer = Tokenizer::new("a = 1 AND b > 2 OR c <= 3".to_string());
		let tokens = tokenizer.tokenize();
		assert!(tokens.contains(&Token::Equals));
		assert!(tokens.contains(&Token::And));
		assert!(tokens.contains(&Token::GreaterThan));
		assert!(tokens.contains(&Token::Or));
		assert!(tokens.contains(&Token::LessThanOrEqual));
	}

	#[test]
	fn test_tokenize_create_table() {
		let tokenizer = Tokenizer::new("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Create);
		assert_eq!(tokens[1], Token::Table);
		assert!(tokens.contains(&Token::Integer));
		assert!(tokens.contains(&Token::Primary));
		assert!(tokens.contains(&Token::Key));
		assert!(tokens.contains(&Token::Text));
	}

	#[test]
	fn test_tokenize_literals() {
		let tokenizer = Tokenizer::new("SELECT 42, 3.14, 'hello'".to_string());
		let tokens = tokenizer.tokenize();
		assert!(tokens.contains(&Token::IntegerLiteral));
		assert!(tokens.contains(&Token::FloatLiteral));
		assert!(tokens.contains(&Token::StringLiteral));
	}

	#[test]
	fn test_tokenize_joins() {
		let tokenizer = Tokenizer::new("SELECT * FROM a INNER JOIN b ON a.id = b.id".to_string());
		let tokens = tokenizer.tokenize();
		assert!(tokens.contains(&Token::Inner));
		assert!(tokens.contains(&Token::Join));
		assert!(tokens.contains(&Token::On));
	}

	#[test]
	fn test_tokenize_savepoint() {
		let tokenizer = Tokenizer::new("SAVEPOINT sp1".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Savepoint);
		assert_eq!(tokens[1], Token::Identifier);
	}

	#[test]
	fn test_tokenize_release() {
		let tokenizer = Tokenizer::new("RELEASE SAVEPOINT sp1".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Release);
		assert_eq!(tokens[1], Token::Savepoint);
		assert_eq!(tokens[2], Token::Identifier);
	}

	#[test]
	fn test_tokenize_rollback_to() {
		let tokenizer = Tokenizer::new("ROLLBACK TO SAVEPOINT sp1".to_string());
		let tokens = tokenizer.tokenize();
		assert_eq!(tokens[0], Token::Rollback);
		assert_eq!(tokens[1], Token::To);
		assert_eq!(tokens[2], Token::Savepoint);
		assert_eq!(tokens[3], Token::Identifier);
	}
}
