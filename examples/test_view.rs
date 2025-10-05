use epiloglite::{Database, ExecutionResult, Result};

fn main() -> Result<()> {
    let mut db = Database::open(":memory:")?;

    // Create base table
    db.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)")?;
    db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")?;

    // Create a view
    db.execute("CREATE VIEW young_users AS SELECT * FROM users")?;

    // Query the view
    let result = db.execute("SELECT * FROM young_users")?;
    match result {
        ExecutionResult::Select { rows, columns } => {
            println!("Columns: {:?}", columns);
            println!("Number of rows: {}", rows.len());
            for (i, row) in rows.iter().enumerate() {
                println!("Row {}: {:?}", i, row);
                for (j, val) in row.iter().enumerate() {
                    println!("  [{}] = '{}'", j, val);
                }
            }
        }
        _ => panic!("Expected Select result"),
    }

    db.close()?;
    Ok(())
}
