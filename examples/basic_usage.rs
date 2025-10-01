/// Basic usage example for EpilogLite

use epiloglite::{Database, Result, EPILOGLITE_VERSION};

fn main() -> Result<()> {
	println!("EpilogLite version {}", EPILOGLITE_VERSION);
	println!("==========================================");

	// Open an in-memory database
	println!("\nOpening in-memory database...");
	let mut db = Database::open(":memory:")?;
	println!("Database opened successfully!");
	println!("Database path: {}", db.path());

	// Create a table
	println!("\nCreating table...");
	let result = db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)");
	match result {
		Ok(_) => println!("Table created successfully!"),
		Err(e) => println!("Error creating table: {}", e),
	}

	// Insert some data
	println!("\nInserting data...");
	let result = db.execute("INSERT INTO users VALUES (1, 'Alice', 30)");
	match result {
		Ok(_) => println!("Data inserted successfully!"),
		Err(e) => println!("Error inserting data: {}", e),
	}

	// Query data
	println!("\nQuerying data...");
	let result = db.execute("SELECT * FROM users");
	match result {
		Ok(_) => println!("Query executed successfully!"),
		Err(e) => println!("Error executing query: {}", e),
	}

	// Update data
	println!("\nUpdating data...");
	let result = db.execute("UPDATE users SET age = 31 WHERE id = 1");
	match result {
		Ok(_) => println!("Data updated successfully!"),
		Err(e) => println!("Error updating data: {}", e),
	}

	// Transactions
	println!("\nTesting transactions...");
	db.execute("BEGIN")?;
	println!("Transaction started");
	db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")?;
	println!("Data inserted in transaction");
	db.execute("COMMIT")?;
	println!("Transaction committed");

	// Close the database
	println!("\nClosing database...");
	db.close()?;
	println!("Database closed successfully!");

	println!("\n==========================================");
	println!("All operations completed successfully!");

	Ok(())
}
