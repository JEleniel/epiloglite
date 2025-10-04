//! Basic no-std example for EpilogLite
//!
//! This example demonstrates using EpilogLite in a no_std environment.
//! It shows basic database operations without requiring the standard library.
//!
//! Note: This is a conceptual example. To actually run this on embedded hardware,
//! you would need to configure the target platform, allocator, and startup code.

#![no_std]

extern crate alloc;

use alloc::string::ToString;

// Example function showing basic database usage in no-std
pub fn run_example() -> Result<(), epiloglite::Error> {
    // Create an in-memory database
    let mut db = epiloglite::Database::new()?;

    // Create a table
    db.execute("CREATE TABLE sensors (id INTEGER, value INTEGER, name TEXT)")?;

    // Insert some data
    db.execute("INSERT INTO sensors VALUES (1, 23, 'temperature')")?;
    db.execute("INSERT INTO sensors VALUES (2, 65, 'humidity')")?;
    db.execute("INSERT INTO sensors VALUES (3, 1013, 'pressure')")?;

    // Query the data
    let result = db.execute("SELECT * FROM sensors WHERE value > 50")?;
    
    // Verify it worked
    match result {
        epiloglite::ExecutionResult::Select { rows, .. } => {
            assert!(rows.len() > 0);
        }
        _ => return Err(epiloglite::Error::Internal("Unexpected result".to_string())),
    }

    Ok(())
}

// Example using query builder
pub fn query_builder_example() -> Result<(), epiloglite::Error> {
    use epiloglite::{SelectBuilder, QueryBuilder};

    let mut db = epiloglite::Database::new()?;
    
    db.execute("CREATE TABLE devices (id INTEGER, status TEXT)")?;
    db.execute("INSERT INTO devices VALUES (1, 'active')")?;
    db.execute("INSERT INTO devices VALUES (2, 'inactive')")?;

    // Use query builder for type-safe queries
    let query = SelectBuilder::new()
        .column("id")
        .column("status")
        .from("devices")
        .where_clause("status = 'active'")
        .build()?;

    db.execute(&query)?;

    Ok(())
}

// Memory-constrained example
pub fn memory_constrained_example() -> Result<(), epiloglite::Error> {
    let mut db = epiloglite::Database::new()?;
    
    // Create a simple table
    db.execute("CREATE TABLE log (id INTEGER, msg TEXT)")?;
    
    // Insert data one at a time to minimize memory usage
    for i in 0..10 {
        let sql = alloc::format!("INSERT INTO log VALUES ({}, 'Message {}')", i, i);
        db.execute(&sql)?;
    }

    Ok(())
}
