/// Integration tests for EpilogLite

use epiloglite::{Database, ExecutionResult, Result};

#[test]
fn test_complete_workflow() -> Result<()> {
	// Open database
	let mut db = Database::open(":memory:")?;

	// Create a table
	db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")?;

	// Insert data
	db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")?;
	db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")?;
	db.execute("INSERT INTO users VALUES (3, 'Charlie', 35)")?;

	// Query data
	let result = db.execute("SELECT * FROM users")?;
	match result {
		ExecutionResult::Select { rows, columns } => {
			assert_eq!(columns.len(), 1); // "*"
			assert_eq!(rows.len(), 3);
		}
		_ => panic!("Expected Select result"),
	}

	// Update data
	let result = db.execute("UPDATE users SET age = 31 WHERE id = 1")?;
	match result {
		ExecutionResult::RowsAffected(n) => {
			assert!(n > 0);
		}
		_ => panic!("Expected RowsAffected result"),
	}

	// Delete data
	let result = db.execute("DELETE FROM users WHERE id = 3")?;
	match result {
		ExecutionResult::RowsAffected(n) => {
			assert!(n > 0);
		}
		_ => panic!("Expected RowsAffected result"),
	}

	// Close database
	db.close()?;

	Ok(())
}

#[test]
fn test_transactions() -> Result<()> {
	let mut db = Database::open(":memory:")?;

	// Create table
	db.execute("CREATE TABLE accounts (id INTEGER, balance INTEGER)")?;
	db.execute("INSERT INTO accounts VALUES (1, 100)")?;

	// Start transaction
	db.execute("BEGIN")?;
	db.execute("UPDATE accounts SET balance = 150 WHERE id = 1")?;
	db.execute("COMMIT")?;

	// Start another transaction and rollback
	db.execute("BEGIN")?;
	db.execute("UPDATE accounts SET balance = 200 WHERE id = 1")?;
	db.execute("ROLLBACK")?;

	db.close()?;

	Ok(())
}

#[test]
fn test_multiple_tables() -> Result<()> {
	let mut db = Database::open(":memory:")?;

	// Create multiple tables
	db.execute("CREATE TABLE users (id INTEGER, name TEXT)")?;
	db.execute("CREATE TABLE posts (id INTEGER, user_id INTEGER, content TEXT)")?;

	// Insert into both tables
	db.execute("INSERT INTO users VALUES (1, 'Alice')")?;
	db.execute("INSERT INTO posts VALUES (1, 1, 'Hello World')")?;

	// Query both tables
	db.execute("SELECT * FROM users")?;
	db.execute("SELECT * FROM posts")?;

	db.close()?;

	Ok(())
}

#[test]
fn test_error_handling() {
	let mut db = Database::open(":memory:").unwrap();

	// Try to insert into non-existent table
	let result = db.execute("INSERT INTO nonexistent VALUES (1, 'test')");
	assert!(result.is_err());

	// Try to select from non-existent table
	let result = db.execute("SELECT * FROM nonexistent");
	assert!(result.is_err());

	// Try to create duplicate table
	db.execute("CREATE TABLE users (id INTEGER)").unwrap();
	let result = db.execute("CREATE TABLE users (id INTEGER)");
	assert!(result.is_err());
}
