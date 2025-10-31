/// Integration tests for stored procedures

use epiloglite::{Error, ExecutionResult, Processor};

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_create_simple_procedure() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc() BEGIN END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
	assert!(matches!(result.unwrap(), ExecutionResult::Success));
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_create_procedure_with_parameters() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE add_user(IN name TEXT, IN age INTEGER) BEGIN END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_create_procedure_with_out_parameter() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE get_count(OUT total INTEGER) BEGIN END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_drop_procedure() {
	let mut processor = Processor::new();
	
	// Create procedure
	processor.execute("CREATE PROCEDURE test_proc() BEGIN END").unwrap();
	
	// Drop procedure
	let result = processor.execute("DROP PROCEDURE test_proc");
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_drop_nonexistent_procedure() {
	let mut processor = Processor::new();
	
	let result = processor.execute("DROP PROCEDURE nonexistent");
	assert!(result.is_err());
	assert!(matches!(result.unwrap_err(), Error::NotFound(_)));
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_call_simple_procedure() {
	let mut processor = Processor::new();
	
	// Create procedure
	processor.execute("CREATE PROCEDURE test_proc() BEGIN END").unwrap();
	
	// Call procedure
	let result = processor.execute("CALL test_proc()");
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_call_procedure_with_arguments() {
	let mut processor = Processor::new();
	
	// Create procedure with parameters
	processor.execute("CREATE PROCEDURE add_user(IN name TEXT, IN age INTEGER) BEGIN END").unwrap();
	
	// Call procedure with arguments
	let result = processor.execute("CALL add_user('Alice', 30)");
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_call_nonexistent_procedure() {
	let mut processor = Processor::new();
	
	let result = processor.execute("CALL nonexistent()");
	assert!(result.is_err());
	assert!(matches!(result.unwrap_err(), Error::NotFound(_)));
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_call_procedure_wrong_argument_count() {
	let mut processor = Processor::new();
	
	// Create procedure with 2 parameters
	processor.execute("CREATE PROCEDURE test_proc(IN x INTEGER, IN y INTEGER) BEGIN END").unwrap();
	
	// Call with wrong number of arguments
	let result = processor.execute("CALL test_proc(1)");
	assert!(result.is_err());
	assert!(matches!(result.unwrap_err(), Error::InvalidOperation(_)));
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_declare() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc() BEGIN DECLARE x INTEGER; END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_set() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc() BEGIN DECLARE x INTEGER; SET x = 10; END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_if() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc(IN x INTEGER) BEGIN IF x > 0 THEN SET x = 1; END IF; END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_while() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc() BEGIN DECLARE x INTEGER; SET x = 0; WHILE x < 10 LOOP SET x = x + 1; END LOOP; END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_return() {
	let mut processor = Processor::new();
	
	let sql = "CREATE PROCEDURE test_proc() BEGIN RETURN 42; END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_duplicate_procedure() {
	let mut processor = Processor::new();
	
	processor.execute("CREATE PROCEDURE test_proc() BEGIN END").unwrap();
	
	let result = processor.execute("CREATE PROCEDURE test_proc() BEGIN END");
	assert!(result.is_err());
	assert!(matches!(result.unwrap_err(), Error::AlreadyExists(_)));
}

#[test]
#[ignore = "Stored procedures feature not yet fully implemented"]
fn test_procedure_with_sql_statement() {
	let mut processor = Processor::new();
	
	// First create a table
	processor.execute("CREATE TABLE users (id INTEGER, name TEXT)").unwrap();
	
	// Create procedure that uses SQL
	let sql = "CREATE PROCEDURE add_user(IN name TEXT) BEGIN INSERT INTO users VALUES (1, name); END";
	let result = processor.execute(sql);
	
	assert!(result.is_ok());
}
