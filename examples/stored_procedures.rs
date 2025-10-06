/// Example demonstrating stored procedures in EpilogLite

use epiloglite::{Database, ExecutionResult, Result};

fn main() -> Result<()> {
	println!("=== EpilogLite Stored Procedures Example ===\n");

	// Open an in-memory database
	let mut db = Database::open(":memory:")?;

	// Create a users table
	println!("1. Creating users table...");
	db.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)")?;

	// Create a simple procedure
	println!("2. Creating a simple procedure...");
	db.execute(
		"CREATE PROCEDURE add_user(IN user_name TEXT, IN user_age INTEGER) 
		BEGIN 
			INSERT INTO users VALUES (1, user_name, user_age);
		END",
	)?;
	println!("   ✓ Procedure 'add_user' created\n");

	// Call the procedure
	println!("3. Calling the procedure...");
	match db.execute("CALL add_user('Alice', 30)") {
		Ok(_) => println!("   ✓ Procedure executed successfully\n"),
		Err(e) => println!("   ✗ Error: {}\n", e),
	}

	// Create a procedure with control flow
	println!("4. Creating procedure with IF statement...");
	db.execute(
		"CREATE PROCEDURE categorize_age(IN age INTEGER)
		BEGIN
			DECLARE category TEXT;
			IF age < 18 THEN
				SET category = 'minor';
			ELSE
				SET category = 'adult';
			END IF;
		END",
	)?;
	println!("   ✓ Procedure 'categorize_age' created\n");

	// Create a procedure with a loop
	println!("5. Creating procedure with WHILE loop...");
	db.execute(
		"CREATE PROCEDURE count_to_ten()
		BEGIN
			DECLARE counter INTEGER;
			SET counter = 1;
			WHILE counter <= 10 LOOP
				SET counter = counter + 1;
			END LOOP;
		END",
	)?;
	println!("   ✓ Procedure 'count_to_ten' created\n");

	// Create a procedure with OUT parameter
	println!("6. Creating procedure with OUT parameter...");
	db.execute(
		"CREATE PROCEDURE get_user_count(OUT total INTEGER)
		BEGIN
			DECLARE count_val INTEGER;
			SET total = count_val;
		END",
	)?;
	println!("   ✓ Procedure 'get_user_count' created\n");

	// Create a procedure with error handling
	println!("7. Creating procedure with error signaling...");
	db.execute(
		"CREATE PROCEDURE validate_age(IN age INTEGER)
		BEGIN
			IF age < 0 THEN
				SIGNAL SQLSTATE '45000';
			END IF;
		END",
	)?;
	println!("   ✓ Procedure 'validate_age' created\n");

	// Drop a procedure
	println!("8. Dropping a procedure...");
	db.execute("DROP PROCEDURE count_to_ten")?;
	println!("   ✓ Procedure 'count_to_ten' dropped\n");

	// Try to call a non-existent procedure
	println!("9. Attempting to call non-existent procedure...");
	match db.execute("CALL count_to_ten()") {
		Ok(_) => println!("   Unexpected success"),
		Err(e) => println!("   ✓ Expected error: {}\n", e),
	}

	// Verify data was inserted
	println!("10. Verifying data...");
	match db.execute("SELECT * FROM users") {
		Ok(ExecutionResult::Select { rows, .. }) => {
			println!("   Users in database: {} row(s)", rows.len());
			for row in rows {
				println!("   - {:?}", row);
			}
		}
		_ => println!("   No data or error"),
	}

	println!("\n=== Example completed successfully! ===");
	println!("\nNote: Full procedure execution (interpreting control flow and");
	println!("SQL statements within procedures) will be available in a future version.");
	println!("Currently, procedures are parsed, stored, and validated.");

	Ok(())
}
