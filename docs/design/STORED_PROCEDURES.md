# Stored Procedures in EpilogLite

## Overview

Stored procedures are named blocks of SQL and procedural code that can be stored in the database and executed on demand. They provide encapsulation, reusability, and improved performance for complex database operations.

## Syntax

### Creating a Stored Procedure

```sql
CREATE PROCEDURE procedure_name (
    [IN | OUT | INOUT] parameter_name data_type
    [, ...]
)
BEGIN
    -- Procedure body statements
END
```

**Parameters:**
- `IN`: Input parameter (default if mode is not specified)
- `OUT`: Output parameter
- `INOUT`: Parameter that can be both input and output

**Data Types:**
- `INTEGER` - 32-bit integer
- `TEXT` - UTF-8 string
- `REAL` - 32-bit floating point
- `BLOB` - Binary data
- `BOOLEAN` - Boolean value

### Dropping a Stored Procedure

```sql
DROP PROCEDURE procedure_name
```

### Calling a Stored Procedure

```sql
CALL procedure_name(argument1, argument2, ...)
```

## Procedure Body Statements

### Variable Declaration

```sql
DECLARE variable_name data_type [DEFAULT default_value];
```

Example:
```sql
DECLARE counter INTEGER DEFAULT 0;
DECLARE user_name TEXT;
```

### Variable Assignment

```sql
SET variable_name = expression;
```

Example:
```sql
SET counter = counter + 1;
SET user_name = 'Alice';
```

### Conditional Execution (IF-THEN-ELSE)

```sql
IF condition THEN
    -- statements
[ELSE
    -- statements]
END IF;
```

Example:
```sql
IF age >= 18 THEN
    SET status = 'adult';
ELSE
    SET status = 'minor';
END IF;
```

### Looping (WHILE)

```sql
WHILE condition LOOP
    -- statements
END LOOP;
```

Example:
```sql
DECLARE i INTEGER DEFAULT 0;
WHILE i < 10 LOOP
    SET i = i + 1;
END LOOP;
```

### Return Statement

```sql
RETURN [expression];
```

Example:
```sql
RETURN total_count;
RETURN;  -- Return without a value
```

### Error Signaling

```sql
SIGNAL SQLSTATE 'sqlstate_value' [SET MESSAGE_TEXT = 'error_message'];
```

Example:
```sql
SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid operation';
```

### Embedded SQL Statements

Procedures can contain any standard SQL statement:
- `SELECT`
- `INSERT`
- `UPDATE`
- `DELETE`

Example:
```sql
INSERT INTO users (name, age) VALUES (user_name, user_age);
SELECT COUNT(*) INTO total FROM users;
```

## Examples

### Simple Procedure

```sql
CREATE PROCEDURE greet()
BEGIN
    SELECT 'Hello, World!';
END
```

Call it:
```sql
CALL greet();
```

### Procedure with IN Parameters

```sql
CREATE PROCEDURE add_user(IN name TEXT, IN age INTEGER)
BEGIN
    INSERT INTO users (name, age) VALUES (name, age);
END
```

Call it:
```sql
CALL add_user('Alice', 30);
```

### Procedure with OUT Parameter

```sql
CREATE PROCEDURE get_user_count(OUT total INTEGER)
BEGIN
    DECLARE count_value INTEGER;
    SELECT COUNT(*) INTO count_value FROM users;
    SET total = count_value;
END
```

Call it:
```sql
CALL get_user_count(@user_count);
```

### Procedure with Control Flow

```sql
CREATE PROCEDURE calculate_discount(IN amount INTEGER, OUT discount INTEGER)
BEGIN
    DECLARE rate INTEGER DEFAULT 0;
    
    IF amount > 1000 THEN
        SET rate = 20;
    ELSE IF amount > 500 THEN
        SET rate = 10;
    ELSE
        SET rate = 5;
    END IF;
    
    SET discount = (amount * rate) / 100;
END
```

### Procedure with Loop

```sql
CREATE PROCEDURE populate_numbers(IN max_value INTEGER)
BEGIN
    DECLARE i INTEGER DEFAULT 1;
    
    WHILE i <= max_value LOOP
        INSERT INTO numbers (value) VALUES (i);
        SET i = i + 1;
    END LOOP;
END
```

### Procedure with Error Handling

```sql
CREATE PROCEDURE transfer_funds(IN from_account INTEGER, IN to_account INTEGER, IN amount INTEGER)
BEGIN
    DECLARE balance INTEGER;
    
    -- Check balance
    SELECT account_balance INTO balance FROM accounts WHERE id = from_account;
    
    IF balance < amount THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Insufficient funds';
    END IF;
    
    -- Perform transfer
    UPDATE accounts SET account_balance = account_balance - amount WHERE id = from_account;
    UPDATE accounts SET account_balance = account_balance + amount WHERE id = to_account;
END
```

## Implementation Details

### Storage

Stored procedures are stored in a `ProcedureRegistry` within the `StorageManager`. The registry maintains:
- Procedure name (unique identifier)
- Parameter definitions (name, type, mode)
- Procedure body (list of statements)

### Execution Context

When a procedure is called, a `ProcedureContext` is created that maintains:
- Variable values (declared within the procedure)
- Parameter values (passed by the caller)

### Statement Parsing

The parser recognizes:
1. `CREATE PROCEDURE` - Creates and stores a procedure definition
2. `DROP PROCEDURE` - Removes a procedure from the registry
3. `CALL` - Executes a stored procedure

### Future Enhancements

The current implementation provides the foundation for stored procedures. Future enhancements may include:

1. **Full Execution Engine**: Currently, procedure bodies are parsed but not fully executed. A complete execution engine will be added to interpret control flow and SQL statements.

2. **Transaction Support**: Automatic transaction management within procedures.

3. **Exception Handlers**: `DECLARE HANDLER` for catching and handling errors.

4. **Cursor Support**: `DECLARE CURSOR` for iterating over query results.

5. **Function Support**: Similar to procedures but with return values that can be used in expressions.

6. **Procedure Dependencies**: Tracking which tables and other procedures are used.

7. **Debugging Support**: Step-through debugging and breakpoints.

8. **Performance Optimization**: Compiled procedure bytecode for faster execution.

## Limitations

Current limitations (to be addressed in future versions):

1. Procedure bodies are parsed but not yet fully executed
2. Variable scoping is basic
3. No support for nested procedure calls yet
4. Limited error handling
5. No support for result sets from SELECT statements within procedures
6. No support for REPEAT...UNTIL or other loop types
7. No support for CASE statements
8. No support for cursors

## Error Handling

The following errors can occur:

- `AlreadyExists`: Attempting to create a procedure with a name that already exists
- `NotFound`: Attempting to drop or call a non-existent procedure
- `InvalidOperation`: Calling a procedure with the wrong number of arguments
- `Syntax`: Parse errors in procedure definition

## Best Practices

1. **Naming**: Use descriptive names with a consistent naming convention (e.g., `verb_noun` like `get_user_count`)

2. **Parameters**: Clearly specify parameter modes (IN, OUT, INOUT)

3. **Error Handling**: Use SIGNAL to report errors with meaningful messages

4. **Comments**: Document complex logic within procedure bodies

5. **Testing**: Test procedures with various input values and edge cases

6. **Security**: Be cautious with procedures that perform data modifications

7. **Performance**: Avoid unnecessary complexity in frequently-called procedures

## Compatibility

EpilogLite's stored procedure syntax is inspired by:
- MySQL/MariaDB stored procedure syntax
- PostgreSQL PL/pgSQL
- SQL standard (SQL/PSM)

Key differences from other systems:
- Simplified syntax for ease of use
- No support for some advanced features (yet)
- String-based SQL embedding (not fully compiled)

## See Also

- [ARCHITECTURE.md](ARCHITECTURE.md) - Overall system architecture
- [QUERYPLANNER.md](QUERYPLANNER.md) - Query optimization
- [TRANSACTIONS.md](TRANSACTIONS.md) - Transaction handling
- [TODO.md](../agents/TODO.md) - Phase 32 requirements
