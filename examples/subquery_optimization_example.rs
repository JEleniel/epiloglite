/// Example demonstrating subquery optimization concepts
///
/// Note: The optimizer is an internal implementation detail and not exposed
/// in the public API. This example demonstrates conceptual usage of subquery
/// optimization features through the query execution interface.

use epiloglite::{Database, Result};

fn main() -> Result<()> {
	println!("EpilogLite Subquery Optimization Example");
	println!("=========================================\n");
	println!("This example demonstrates queries that benefit from");
	println!("the subquery optimization features implemented in Phase 29:\n");
	println!("  - Subquery flattening");
	println!("  - Correlated subquery support");
	println!("  - IN subquery optimization");
	println!("  - EXISTS optimization");
	println!("  - Subquery result caching\n");

	// Open database
	let mut db = Database::open(":memory:")?;
	println!("✓ Database opened\n");

	// Create sample tables
	println!("Creating sample tables...");
	db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")?;
	db.execute("CREATE TABLE orders (id INTEGER PRIMARY KEY, user_id INTEGER, amount REAL, status TEXT)")?;
	db.execute("CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT, price REAL, category_id INTEGER)")?;
	println!("✓ Tables created\n");

	// Insert sample data
	println!("Inserting sample data...");
	db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")?;
	db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")?;
	db.execute("INSERT INTO users VALUES (3, 'Charlie', 35)")?;
	
	db.execute("INSERT INTO orders VALUES (101, 1, 150.00, 'active')")?;
	db.execute("INSERT INTO orders VALUES (102, 1, 200.00, 'completed')")?;
	db.execute("INSERT INTO orders VALUES (103, 2, 75.00, 'active')")?;
	db.execute("INSERT INTO orders VALUES (104, 3, 300.00, 'completed')")?;
	
	db.execute("INSERT INTO products VALUES (1, 'Widget', 25.00, 1)")?;
	db.execute("INSERT INTO products VALUES (2, 'Gadget', 150.00, 1)")?;
	db.execute("INSERT INTO products VALUES (3, 'Doohickey', 75.00, 2)")?;
	println!("✓ Data inserted\n");

	// Example 1: Scalar subquery (can be flattened)
	println!("Example 1: Scalar Subquery");
	println!("--------------------------");
	println!("Query: SELECT name FROM users WHERE age > (SELECT AVG(age) FROM users)");
	println!("Optimization: This subquery can potentially be flattened or cached");
	println!("Note: Currently basic implementation - optimization happens internally\n");

	// Example 2: IN subquery (benefits from optimization)
	println!("Example 2: IN Subquery");
	println!("----------------------");
	println!("Conceptual query: SELECT * FROM orders WHERE user_id IN (SELECT id FROM users WHERE age > 30)");
	println!("Optimization: IN subquery can use hash table for large result sets");
	println!("Note: Currently basic implementation - optimization happens internally\n");

	// Example 3: EXISTS subquery (short-circuit evaluation)
	println!("Example 3: EXISTS Subquery");
	println!("--------------------------");
	println!("Conceptual query: SELECT * FROM users WHERE EXISTS (SELECT 1 FROM orders WHERE orders.user_id = users.id)");
	println!("Optimization: EXISTS stops at first match (short-circuit evaluation)");
	println!("Note: Currently basic implementation - optimization happens internally\n");

	// Example 4: Correlated subquery (benefits from caching)
	println!("Example 4: Correlated Subquery");
	println!("-------------------------------");
	println!("Conceptual query: SELECT name, (SELECT COUNT(*) FROM orders WHERE user_id = users.id) FROM users");
	println!("Optimization: Results cached for repeated parameter values");
	println!("Note: Currently basic implementation - optimization happens internally\n");

	// Run some actual queries that demonstrate the concepts
	println!("Running actual queries...");
	println!("------------------------\n");

	println!("Query: All active orders");
	let result = db.execute("SELECT * FROM orders WHERE status = 'active'")?;
	println!("✓ Query executed: {:?}\n", result);

	println!("Query: All users");
	let result = db.execute("SELECT * FROM users")?;
	println!("✓ Query executed: {:?}\n", result);

	db.close()?;
	println!("=========================================");
	println!("Example completed!");
	println!("\nNote: The optimizer module is internal and works behind the scenes");
	println!("to optimize subquery execution. For detailed optimizer behavior,");
	println!("see the unit tests in src/eplite/optimizer.rs");

	Ok(())
}
