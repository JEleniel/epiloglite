/// SQL tokenizer - breaks SQL text into tokens

use logos::Logos;

/// SQL token types
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	// Keywords
	#[token("SELECT", ignore(ascii_case))]
	Select,
	#[token("FROM", ignore(ascii_case))]
	From,
	#[token("WHERE", ignore(ascii_case))]
	Where,
	#[token("INSERT", ignore(ascii_case))]
	Insert,
	#[token("UPDATE", ignore(ascii_case))]
	Update,
	#[token("DELETE", ignore(ascii_case))]
	Delete,
	#[token("CREATE", ignore(ascii_case))]
	Create,
	#[token("TABLE", ignore(ascii_case))]
	Table,
	#[token("INDEX", ignore(ascii_case))]
	Index,

	// Symbols
	#[token("(")]
	LeftParen,
	#[token(")")]
	RightParen,
	#[token(",")]
	Comma,
	#[token(";")]
	Semicolon,
	#[token("*")]
	Star,
	#[token("=")]
	Equals,

	// Identifiers and literals
	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
	Identifier,
	#[regex(r#""[^"]*""#)]
	StringLiteral,
	#[regex(r"[0-9]+")]
	IntegerLiteral,
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
		let mut lex = Token::lexer(&self.source);

		while let Some(token) = lex.next() {
			if let Ok(token) = token {
				tokens.push(token);
			}
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
}
