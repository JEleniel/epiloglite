/// Adversarial and security tests for EpilogLite
/// Tests malformed SQL, injection attacks, keywords as identifiers, edge cases

use epiloglite::{Database, Result};

#[test]
fn test_malformed_sql_missing_table_name() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Missing table name in CREATE TABLE
	let result = db.execute("CREATE TABLE");
	assert!(result.is_err(), "Should fail with missing table name");
	
	// Missing table name in INSERT
	let result = db.execute("INSERT INTO VALUES (1, 2)");
	assert!(result.is_err(), "Should fail with missing table name");
	
	// Missing table name in SELECT
	let result = db.execute("SELECT * FROM");
	assert!(result.is_err(), "Should fail with missing table name");
}

#[test]
fn test_malformed_sql_invalid_syntax() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Random gibberish
	let result = db.execute("BLAH BLAH BLAH");
	assert!(result.is_err(), "Should fail with invalid syntax");
	
	// Incomplete statement
	let result = db.execute("CREATE");
	assert!(result.is_err(), "Should fail with incomplete statement");
	
	// Missing parentheses
	let result = db.execute("CREATE TABLE users id INTEGER");
	assert!(result.is_err(), "Should fail with missing parentheses");
	
	// Mismatched parentheses
	let result = db.execute("CREATE TABLE users (id INTEGER");
	assert!(result.is_err(), "Should fail with mismatched parentheses");
	
	// Extra tokens
	let result = db.execute("CREATE TABLE users (id INTEGER) EXTRA STUFF");
	assert!(result.is_err(), "Should fail with extra tokens");
}

#[test]
fn test_malformed_sql_missing_columns() {
	let mut db = Database::open(":memory:").unwrap();
	
	// No columns in CREATE TABLE
	let result = db.execute("CREATE TABLE users ()");
	assert!(result.is_err(), "Should fail with no columns");
	
	// Missing column definition
	let result = db.execute("CREATE TABLE users (id, name TEXT)");
	assert!(result.is_err(), "Should fail with incomplete column definition");
}

#[test]
fn test_malformed_sql_invalid_types() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Invalid data type
	let result = db.execute("CREATE TABLE users (id INVALID_TYPE)");
	assert!(result.is_err(), "Should fail with invalid data type");
	
	// Missing data type
	let result = db.execute("CREATE TABLE users (id)");
	assert!(result.is_err(), "Should fail with missing data type");
}

#[test]
fn test_sql_injection_attempts() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Create a test table
	db.execute("CREATE TABLE users (id INTEGER, name TEXT, password TEXT)").unwrap();
	db.execute("INSERT INTO users VALUES (1, 'admin', 'secret')").unwrap();
	
	// Classic SQL injection: ' OR '1'='1
	// This should be treated as a literal string value, not executed as SQL
	let result = db.execute("INSERT INTO users VALUES (2, '' OR '1'='1', 'test')");
	// Should either succeed (treating it as a string) or fail safely (syntax error)
	// but never execute the injection
	
	// Comment injection: --
	let result = db.execute("INSERT INTO users VALUES (3, 'test-- DROP TABLE users', 'pwd')");
	// Should handle safely
	
	// Semicolon to chain commands
	let result = db.execute("INSERT INTO users VALUES (4, 'test'); DROP TABLE users; --', 'pwd')");
	// Should fail or treat as single statement, not execute DROP
	assert!(result.is_err() || db.execute("SELECT * FROM users").is_ok(), 
		"Table should still exist after injection attempt");
}

#[test]
fn test_keywords_as_identifiers() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Reserved keywords as table names should be handled
	// These should ideally fail or require quoting
	let result = db.execute("CREATE TABLE SELECT (id INTEGER)");
	assert!(result.is_err(), "Should not allow SELECT as table name");
	
	let result = db.execute("CREATE TABLE INSERT (id INTEGER)");
	assert!(result.is_err(), "Should not allow INSERT as table name");
	
	let result = db.execute("CREATE TABLE DELETE (id INTEGER)");
	assert!(result.is_err(), "Should not allow DELETE as table name");
	
	let result = db.execute("CREATE TABLE TABLE (id INTEGER)");
	assert!(result.is_err(), "Should not allow TABLE as table name");
	
	// Keywords as column names
	let result = db.execute("CREATE TABLE test (SELECT INTEGER)");
	assert!(result.is_err(), "Should not allow SELECT as column name");
	
	let result = db.execute("CREATE TABLE test (FROM INTEGER)");
	assert!(result.is_err(), "Should not allow FROM as column name");
}

#[test]
fn test_edge_case_empty_strings() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Empty SQL string
	let result = db.execute("");
	assert!(result.is_err(), "Should fail with empty string");
	
	// Only whitespace
	let result = db.execute("   ");
	assert!(result.is_err(), "Should fail with only whitespace");
	
	// Only newlines and tabs
	let result = db.execute("\n\t\n");
	assert!(result.is_err(), "Should fail with only whitespace characters");
}

#[test]
fn test_edge_case_very_long_names() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Very long table name (should handle gracefully, might have limits)
	let long_name = "a".repeat(1000);
	let sql = format!("CREATE TABLE {} (id INTEGER)", long_name);
	let result = db.execute(&sql);
	// Either succeeds or fails gracefully, shouldn't panic
	
	// Very long column name
	let long_col = "b".repeat(1000);
	let sql = format!("CREATE TABLE test ({} INTEGER)", long_col);
	let result = db.execute(&sql);
	// Should handle without panic
}

#[test]
fn test_edge_case_special_characters() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Special characters in identifiers (without quotes, should fail)
	let result = db.execute("CREATE TABLE test-table (id INTEGER)");
	assert!(result.is_err(), "Should fail with hyphen in unquoted identifier");
	
	let result = db.execute("CREATE TABLE test.table (id INTEGER)");
	assert!(result.is_err(), "Should fail with dot in unquoted identifier");
	
	let result = db.execute("CREATE TABLE test@table (id INTEGER)");
	assert!(result.is_err(), "Should fail with @ in unquoted identifier");
	
	// Numbers at start of identifier (should fail)
	let result = db.execute("CREATE TABLE 123table (id INTEGER)");
	assert!(result.is_err(), "Should fail with number at start of identifier");
}

#[test]
fn test_edge_case_case_sensitivity() {
	let mut db = Database::open(":memory:").unwrap();
	
	// SQL keywords should be case-insensitive
	db.execute("create table users (id integer)").unwrap();
	db.execute("CREATE TABLE posts (id INTEGER)").unwrap();
	db.execute("CrEaTe TaBlE comments (id InTeGeR)").unwrap();
	
	// Should be able to query with different case
	db.execute("select * from users").unwrap();
	db.execute("SELECT * FROM posts").unwrap();
	db.execute("SeLeCt * FrOm comments").unwrap();
}

#[test]
fn test_edge_case_unicode_characters() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Unicode in string values should work
	db.execute("CREATE TABLE users (id INTEGER, name TEXT)").unwrap();
	db.execute("INSERT INTO users VALUES (1, '日本語')").unwrap();
	db.execute("INSERT INTO users VALUES (2, 'émoji 😀')").unwrap();
	db.execute("INSERT INTO users VALUES (3, 'Кириллица')").unwrap();
	
	// Should be able to retrieve unicode data
	let result = db.execute("SELECT * FROM users").unwrap();
}

#[test]
fn test_edge_case_null_bytes() {
	let mut db = Database::open(":memory:").unwrap();
	db.execute("CREATE TABLE test (data TEXT)").unwrap();
	
	// Null byte in string (should be handled safely)
	let sql_with_null = format!("INSERT INTO test VALUES ('{}')", "test\0data");
	let result = db.execute(&sql_with_null);
	// Should either succeed or fail gracefully, not crash
}

#[test]
fn test_edge_case_nested_quotes() {
	let mut db = Database::open(":memory:").unwrap();
	db.execute("CREATE TABLE test (data TEXT)").unwrap();
	
	// Single quotes within strings
	let result = db.execute("INSERT INTO test VALUES ('O''Brien')");
	// Should handle escaped quotes
	
	let result = db.execute("INSERT INTO test VALUES ('It''s working')");
	// Should handle multiple escapes
}

#[test]
fn test_table_operations_on_nonexistent() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Operations on non-existent tables should fail gracefully
	let result = db.execute("INSERT INTO nonexistent VALUES (1, 2, 3)");
	assert!(result.is_err(), "Should fail inserting into nonexistent table");
	
	let result = db.execute("SELECT * FROM nonexistent");
	assert!(result.is_err(), "Should fail selecting from nonexistent table");
	
	let result = db.execute("UPDATE nonexistent SET col = 1");
	assert!(result.is_err(), "Should fail updating nonexistent table");
	
	let result = db.execute("DELETE FROM nonexistent");
	assert!(result.is_err(), "Should fail deleting from nonexistent table");
}

#[test]
fn test_duplicate_table_creation() {
	let mut db = Database::open(":memory:").unwrap();
	
	// Create table
	db.execute("CREATE TABLE users (id INTEGER)").unwrap();
	
	// Try to create again with same name
	let result = db.execute("CREATE TABLE users (id INTEGER)");
	assert!(result.is_err(), "Should fail creating duplicate table");
	
	// Different schema but same name should still fail
	let result = db.execute("CREATE TABLE users (name TEXT)");
	assert!(result.is_err(), "Should fail creating duplicate table with different schema");
}

#[test]
fn test_column_count_mismatch() {
	let mut db = Database::open(":memory:").unwrap();
	db.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)").unwrap();
	
	// Too few values
	let result = db.execute("INSERT INTO users VALUES (1, 'Alice')");
	assert!(result.is_err(), "Should fail with too few values");
	
	// Too many values
	let result = db.execute("INSERT INTO users VALUES (1, 'Alice', 30, 'extra')");
	assert!(result.is_err(), "Should fail with too many values");
}

#[test]
fn test_concurrent_safety_basic() {
	// Test that basic operations don't panic with multiple operations
	let mut db = Database::open(":memory:").unwrap();
	db.execute("CREATE TABLE test (id INTEGER)").unwrap();
	
	// Multiple inserts
	for i in 0..100 {
		db.execute(&format!("INSERT INTO test VALUES ({})", i)).unwrap();
	}
	
	// Multiple selects
	for _ in 0..100 {
		db.execute("SELECT * FROM test").unwrap();
	}
	
	// Should still work
	let result = db.execute("SELECT * FROM test").unwrap();
}

#[test]
fn test_resource_exhaustion_resistance() {
	let mut db = Database::open(":memory:").unwrap();
	db.execute("CREATE TABLE test (id INTEGER)").unwrap();
	
	// Try to insert many rows (should handle without crashing)
	for i in 0..1000 {
		let _ = db.execute(&format!("INSERT INTO test VALUES ({})", i));
	}
	
	// Should still be functional
	db.execute("SELECT * FROM test").unwrap();
}
