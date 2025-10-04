/// Example demonstrating savepoint functionality in EpilogLite
/// 
/// This example shows how to use SAVEPOINT, RELEASE, and ROLLBACK TO SAVEPOINT
/// statements to create nested transaction points.

use epiloglite::{Database, ExecutionResult, Result};

fn main() -> Result<()> {
    println!("=== EpilogLite Savepoint Example ===\n");

    // Open an in-memory database
    let mut db = Database::open(":memory:")?;
    println!("✓ Opened in-memory database\n");

    // Create a test table
    db.execute("CREATE TABLE accounts (id INTEGER PRIMARY KEY, name TEXT, balance INTEGER)")?;
    println!("✓ Created accounts table\n");

    // Insert initial data
    db.execute("INSERT INTO accounts VALUES (1, 'Alice', 1000)")?;
    db.execute("INSERT INTO accounts VALUES (2, 'Bob', 500)")?;
    println!("✓ Inserted initial accounts\n");

    // Begin a transaction
    db.execute("BEGIN")?;
    println!("✓ Started transaction\n");

    // Create a savepoint before the first operation
    db.execute("SAVEPOINT before_transfer")?;
    println!("✓ Created savepoint 'before_transfer'\n");

    // Simulate a transfer from Alice to Bob
    db.execute("UPDATE accounts SET balance = 900 WHERE id = 1")?;
    db.execute("UPDATE accounts SET balance = 600 WHERE id = 2")?;
    println!("✓ Transferred 100 from Alice to Bob\n");

    // Query current balances
    let result = db.execute("SELECT * FROM accounts")?;
    if let ExecutionResult::Select { rows, .. } = result {
        println!("Current balances:");
        for row in &rows {
            println!("  Account {}: balance = {}", row[0], row[2]);
        }
        println!();
    }

    // Create another savepoint
    db.execute("SAVEPOINT after_first_transfer")?;
    println!("✓ Created savepoint 'after_first_transfer'\n");

    // Make another transfer
    db.execute("UPDATE accounts SET balance = 850 WHERE id = 1")?;
    db.execute("UPDATE accounts SET balance = 650 WHERE id = 2")?;
    println!("✓ Transferred another 50 from Alice to Bob\n");

    // Query balances again
    let result = db.execute("SELECT * FROM accounts")?;
    if let ExecutionResult::Select { rows, .. } = result {
        println!("Balances after second transfer:");
        for row in &rows {
            println!("  Account {}: balance = {}", row[0], row[2]);
        }
        println!();
    }

    // Demonstrate ROLLBACK TO SAVEPOINT
    println!("Rolling back to 'after_first_transfer' savepoint...");
    db.execute("ROLLBACK TO SAVEPOINT after_first_transfer")?;
    println!("✓ Rolled back to savepoint\n");

    // Query balances after rollback
    // Note: In this implementation, savepoints are parsed but not yet enforced
    // The actual rollback behavior will be implemented in a future update
    println!("Note: Savepoints are currently parsed and accepted.");
    println!("Full transaction state tracking will be implemented in a future update.\n");

    // Release a savepoint (commits it)
    db.execute("RELEASE SAVEPOINT before_transfer")?;
    println!("✓ Released savepoint 'before_transfer'\n");

    // Alternative syntax without SAVEPOINT keyword
    db.execute("SAVEPOINT sp1")?;
    println!("✓ Created savepoint 'sp1'\n");
    
    db.execute("RELEASE sp1")?;
    println!("✓ Released savepoint using short syntax\n");
    
    db.execute("SAVEPOINT sp2")?;
    println!("✓ Created savepoint 'sp2'\n");
    
    db.execute("ROLLBACK TO sp2")?;
    println!("✓ Rolled back using short syntax\n");

    // Commit the transaction
    db.execute("COMMIT")?;
    println!("✓ Committed transaction\n");

    // Close the database
    db.close()?;
    println!("✓ Closed database\n");

    println!("=== Example completed successfully ===");
    Ok(())
}
