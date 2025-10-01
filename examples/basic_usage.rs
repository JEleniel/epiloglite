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

	// Try to execute a simple SQL statement (will fail as not implemented yet)
	println!("\nAttempting to execute SQL...");
	match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)") {
		Ok(_) => println!("SQL executed successfully!"),
		Err(e) => println!("SQL execution not yet implemented: {}", e),
	}

	// Close the database
	println!("\nClosing database...");
	db.close()?;
	println!("Database closed successfully!");

	Ok(())
}
