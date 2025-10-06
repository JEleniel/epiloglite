# Positive SQL Test Cases

This directory contains well-formed SQL statements that should parse and execute successfully in EpilogLite.

## Purpose

These test files provide comprehensive coverage of:

-	Valid SQL syntax
-	All supported statement types
-	Optional clause combinations
-	Various data types
-	Complex queries with multiple features

## Test Files

### Basic Statements

-	**select_basic.sql** - Basic SELECT statements with star and column lists
-	**select_where.sql** - SELECT with WHERE clauses
-	**select_order_by.sql** - SELECT with ORDER BY (ASC/DESC)
-	**select_group_by.sql** - SELECT with GROUP BY
-	**select_without_optional_clauses.sql** - SELECT statements without optional clauses
-	**insert_basic.sql** - INSERT statements with and without column specifications
-	**update_basic.sql** - UPDATE statements with and without WHERE
-	**delete_basic.sql** - DELETE statements with WHERE conditions
-	**create_table_basic.sql** - CREATE TABLE with various column types
-	**create_table_constraints.sql** - CREATE TABLE with PRIMARY KEY and constraints

### Advanced Features

-	**select_aggregates.sql** - Aggregate functions (COUNT, SUM, AVG, MIN, MAX)
-	**select_joins.sql** - JOIN operations (INNER, LEFT, RIGHT, CROSS)
-	**select_combinations.sql** - Complex queries combining multiple features
-	**complex_queries.sql** - Advanced multi-table queries with all features

### Transactions and Savepoints

-	**transactions.sql** - BEGIN, COMMIT, ROLLBACK statements
-	**savepoints.sql** - SAVEPOINT, RELEASE, ROLLBACK TO statements

### Data Types

-	**all_data_types.sql** - CREATE TABLE covering all supported data types
	-	NULL, BOOLEAN
	-	INT8, UINT8, INT16, UINT16, INTEGER (INT32), UINT32
	-	BIGINT (INT64), UINT64, INT128, UINT128
	-	REAL (FLOAT32), DOUBLE (FLOAT64)
	-	TEXT, BLOB

### Format Variations

-	**case_variations.sql** - Different case combinations (lowercase, UPPERCASE, MixedCase)
-	**whitespace_variations.sql** - Various whitespace patterns (spaces, tabs, newlines)

## Usage

These files can be used to:

1.	**Verify parser correctness** - Ensure all valid SQL is accepted
2.	**Test execution** - Confirm statements execute without errors
3.	**Regression testing** - Detect when valid SQL becomes invalid
4.	**Documentation** - Serve as examples of supported SQL syntax

## Testing Approach

Each statement in these files should:

-	Parse without syntax errors
-	Execute successfully (given appropriate database state)
-	Return expected results (when applicable)

## Coverage

The test cases cover:

-	✅ All statement types (SELECT, INSERT, UPDATE, DELETE, CREATE TABLE)
-	✅ All optional clauses (WHERE, ORDER BY, GROUP BY, JOIN)
-	✅ Presence and absence of optional clauses
-	✅ All aggregate functions
-	✅ All JOIN types
-	✅ All data types
-	✅ Transaction control
-	✅ Savepoint operations
-	✅ Case insensitivity
-	✅ Whitespace flexibility
