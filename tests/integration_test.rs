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
fn test_aggregate_functions() -> Result<()> {
	let mut db = Database::open(":memory:")?;

	// Create table
	db.execute("CREATE TABLE products (id INTEGER, name TEXT, price INTEGER)")?;
	db.execute("INSERT INTO products VALUES (1, 'Widget', 100)")?;
	db.execute("INSERT INTO products VALUES (2, 'Gadget', 200)")?;
	db.execute("INSERT INTO products VALUES (3, 'Doohickey', 150)")?;

	// Test COUNT(*)
	let result = db.execute("SELECT COUNT(*) FROM products")?;
	match result {
		ExecutionResult::Select { rows, columns } => {
			assert_eq!(columns[0], "COUNT(*)");
			assert_eq!(rows[0][0], "3");
		}
		_ => panic!("Expected Select result"),
	}

	// Test SUM
	let result = db.execute("SELECT SUM(price) FROM products")?;
	match result {
		ExecutionResult::Select { rows, .. } => {
			assert_eq!(rows[0][0], "450");
		}
		_ => panic!("Expected Select result"),
	}

	// Test AVG
	let result = db.execute("SELECT AVG(price) FROM products")?;
	match result {
		ExecutionResult::Select { rows, .. } => {
			let avg: f64 = rows[0][0].parse().unwrap();
			assert!((avg - 150.0).abs() < 0.01);
		}
		_ => panic!("Expected Select result"),
	}

	// Test MIN
	let result = db.execute("SELECT MIN(price) FROM products")?;
	match result {
		ExecutionResult::Select { rows, .. } => {
			assert_eq!(rows[0][0], "100");
		}
		_ => panic!("Expected Select result"),
	}

	// Test MAX
	let result = db.execute("SELECT MAX(price) FROM products")?;
	match result {
		ExecutionResult::Select { rows, .. } => {
			assert_eq!(rows[0][0], "200");
		}
		_ => panic!("Expected Select result"),
	}

	// Test COUNT with WHERE
	let result = db.execute("SELECT COUNT(*) FROM products WHERE price > 100")?;
	match result {
		ExecutionResult::Select { rows, .. } => {
			assert_eq!(rows[0][0], "2");
		}
		_ => panic!("Expected Select result"),
	}

	db.close()?;
	Ok(())
}

#[test]
fn test_disk_persistence() -> Result<()> {
	let test_db_path = "/tmp/test_epiloglite.db";
	
	// Clean up any existing test database
	let _ = std::fs::remove_file(test_db_path);

	// Create database and add data
	{
		let mut db = Database::open(test_db_path)?;
		db.execute("CREATE TABLE products (id INTEGER, name TEXT, price INTEGER)")?;
		db.execute("INSERT INTO products VALUES (1, 'Widget', 100)")?;
		db.execute("INSERT INTO products VALUES (2, 'Gadget', 200)")?;
		db.close()?;
	}

	// Reopen database and verify data persisted
	{
		let mut db = Database::open(test_db_path)?;
		let result = db.execute("SELECT * FROM products")?;
		
		match result {
			ExecutionResult::Select { rows, .. } => {
				assert_eq!(rows.len(), 2, "Expected 2 rows to be persisted");
			}
			_ => panic!("Expected Select result"),
		}
		
		// Add more data
		db.execute("INSERT INTO products VALUES (3, 'Doohickey', 300)")?;
		db.close()?;
	}

	// Reopen again and verify all data
	{
		let mut db = Database::open(test_db_path)?;
		let result = db.execute("SELECT * FROM products")?;
		
		match result {
			ExecutionResult::Select { rows, .. } => {
				assert_eq!(rows.len(), 3, "Expected 3 rows after second insert");
			}
			_ => panic!("Expected Select result"),
		}
		
		db.close()?;
	}

	// Clean up
	std::fs::remove_file(test_db_path)?;

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
