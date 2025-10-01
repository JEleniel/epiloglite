/// Example demonstrating the query builder pattern

use epiloglite::{
	CreateTableBuilder, Database, DeleteBuilder, InsertBuilder, Result, SelectBuilder,
	UpdateBuilder,
};

fn main() -> Result<()> {
	println!("EpilogLite Query Builder Example");
	println!("=================================\n");

	// Open database
	let mut db = Database::open(":memory:")?;

	// Create table using builder
	println!("Creating table with builder...");
	let create_sql = CreateTableBuilder::new()
		.table("employees")
		.column("id", "INTEGER", &["PRIMARY KEY"])
		.simple_column("name", "TEXT")
		.simple_column("department", "TEXT")
		.simple_column("salary", "INTEGER")
		.build()?;

	println!("Generated SQL: {}", create_sql);
	db.execute(&create_sql)?;
	println!("✓ Table created\n");

	// Insert data using builder
	println!("Inserting data with builder...");
	let insert_sql = InsertBuilder::new()
		.into("employees")
		.columns(&["id", "name", "department", "salary"])
		.values(&["1", "'Alice'", "'Engineering'", "75000"])
		.build()?;

	println!("Generated SQL: {}", insert_sql);
	db.execute(&insert_sql)?;

	// Insert more rows
	db.execute(
		&InsertBuilder::new()
			.into("employees")
			.values(&["2", "'Bob'", "'Sales'", "65000"])
			.build()?,
	)?;

	db.execute(
		&InsertBuilder::new()
			.into("employees")
			.values(&["3", "'Charlie'", "'Engineering'", "80000"])
			.build()?,
	)?;

	println!("✓ Data inserted\n");

	// Select data using builder
	println!("Querying data with builder...");
	let select_sql = SelectBuilder::new()
		.select_all()
		.from("employees")
		.where_clause("department = 'Engineering'")
		.order_by("salary")
		.build()?;

	println!("Generated SQL: {}", select_sql);
	let result = db.execute(&select_sql)?;
	println!("✓ Query executed\n");

	// Update data using builder
	println!("Updating data with builder...");
	let update_sql = UpdateBuilder::new()
		.table("employees")
		.set("salary", "85000")
		.where_clause("name = 'Charlie'")
		.build()?;

	println!("Generated SQL: {}", update_sql);
	db.execute(&update_sql)?;
	println!("✓ Data updated\n");

	// Delete data using builder
	println!("Deleting data with builder...");
	let delete_sql = DeleteBuilder::new()
		.from("employees")
		.where_clause("id = 2")
		.build()?;

	println!("Generated SQL: {}", delete_sql);
	db.execute(&delete_sql)?;
	println!("✓ Data deleted\n");

	// Complex query
	println!("Complex query with builder...");
	let complex_sql = SelectBuilder::new()
		.column("name")
		.column("salary")
		.from("employees")
		.where_clause("salary > 70000")
		.order_by("salary")
		.limit(5)
		.build()?;

	println!("Generated SQL: {}", complex_sql);
	db.execute(&complex_sql)?;
	println!("✓ Complex query executed\n");

	db.close()?;

	println!("=================================");
	println!("Query builder example completed!");

	Ok(())
}
