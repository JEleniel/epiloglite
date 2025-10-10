# SQL Test Files

This document provides an overview of the SQL test file collections in the EpilogLite test suite.

## Directory Structure

```text
tests/
├── sql_positive/          # Valid SQL statements (should succeed)
│   ├── README.md         # Documentation for positive tests
│   └── *.sql             # 19 test files with valid SQL
│
└── sql_negative/          # Invalid SQL statements (should fail)
    ├── README.md         # Documentation for negative tests
    └── *.sql             # 15 test files with malformed/adversarial SQL
```

## Overview

### Positive Tests (sql_positive/)

**Total Files:** 19 SQL test files

The positive test directory contains well-formed SQL statements that demonstrate:

-	**Comprehensive statement coverage**: All supported SQL statement types
-	**Optional clause testing**: Presence and absence of WHERE, ORDER BY, GROUP BY, JOIN
-	**Data type coverage**: All supported data types from NULL to BLOB
-	**Feature combinations**: Complex queries using multiple features together
-	**Syntax variations**: Case insensitivity, whitespace flexibility

**Categories:**

1.	**Basic SELECT queries** (5 files)
	+	select_basic.sql
	+	select_where.sql
	+	select_order_by.sql
	+	select_group_by.sql
	+	select_without_optional_clauses.sql

2.	**Advanced SELECT features** (3 files)
	+	select_aggregates.sql
	+	select_joins.sql
	+	select_combinations.sql

3.	**Data modification** (3 files)
	+	insert_basic.sql
	+	update_basic.sql
	+	delete_basic.sql

4.	**Schema definition** (2 files)
	+	create_table_basic.sql
	+	create_table_constraints.sql

5.	**Transactions** (2 files)
	+	transactions.sql
	+	savepoints.sql

6.	**Type coverage** (1 file)
	+	all_data_types.sql

7.	**Format variations** (2 files)
	+	case_variations.sql
	+	whitespace_variations.sql

8.	**Complex scenarios** (1 file)
	+	complex_queries.sql

### Negative Tests (sql_negative/)

**Total Files:** 15 SQL test files

The negative test directory contains malformed and adversarial SQL statements that test:

-	**Error handling**: Syntax errors, type errors, constraint violations
-	**Security**: SQL injection attempts, buffer overflow attempts
-	**Edge cases**: Boundary conditions, unusual inputs
-	**Robustness**: Parser resilience against malformed input

**Categories:**

1.	**Syntax violations** (1 file)
	+	syntax_errors.sql - Missing keywords, wrong order, mismatched parentheses

2.	**Identifier issues** (1 file)
	+	invalid_identifiers.sql - Invalid names, special characters, injection attempts

3.	**Type system** (1 file)
	+	type_errors.sql - Invalid types, missing types, wrong type syntax

4.	**Value literals** (1 file)
	+	malformed_values.sql - Unclosed strings, missing commas, invalid escapes

5.	**Security testing** (1 file)
	+	injection_attempts.sql - SQL injection attack patterns

6.	**Operation errors** (1 file)
	+	invalid_operations.sql - Wrong structure, missing clauses, invalid functions

7.	**Numeric issues** (1 file)
	+	numeric_errors.sql - Invalid number formats, multiple decimals

8.	**Operator misuse** (1 file)
	+	operator_errors.sql - Missing operands, invalid operators

9.	**Aggregate errors** (1 file)
	+	aggregate_errors.sql - Invalid aggregate functions, wrong usage

10.	**JOIN errors** (1 file)
	+	join_errors.sql - Missing ON, invalid JOIN types, incomplete syntax

11.	**Constraint violations** (1 file)
	+	constraint_violations.sql - Duplicate PRIMARY KEY, invalid syntax

12.	**Transaction errors** (1 file)
	+	transaction_errors.sql - Invalid transaction control, savepoint errors

13.	**Edge cases** (1 file)
	+	edge_cases.sql - Empty statements, Unicode, special characters

14.	**Buffer overflow** (1 file)
	+	buffer_overflow_attempts.sql - Extremely long inputs, deep nesting

15.	**Quote handling** (1 file)
	+	quote_escabing_errors.sql - Unescaped quotes, mismatched quotes

## Test Statistics

-	**Total SQL test files**: 34 (19 positive + 15 negative)
-	**Total individual test cases**: 200+ individual SQL statements
-	**Statement types covered**: SELECT, INSERT, UPDATE, DELETE, CREATE TABLE, BEGIN, COMMIT, ROLLBACK, SAVEPOINT, RELEASE
-	**Data types covered**: All 17+ supported types
-	**Optional clauses**: WHERE, ORDER BY, GROUP BY, JOIN (all combinations)
-	**Aggregate functions**: COUNT, SUM, AVG, MIN, MAX
-	**JOIN types**: INNER, LEFT, RIGHT, CROSS

## Usage

### For Manual Testing

```bash
# View a specific test file
cat tests/sql_positive/select_basic.sql

# Count statements in a file
grep -c "^[A-Z]" tests/sql_positive/select_basic.sql
```

### For Automated Testing

These SQL files can be used to create comprehensive test suites:

```rust
// Example: Load and test positive SQL files
use std::fs;

fn test_positive_sql_files() {
    let files = fs::read_dir("tests/sql_positive")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"));
    
    for file in files {
        let content = fs::read_to_string(file.path()).unwrap();
        for statement in parse_sql_statements(&content) {
            // Test that statement parses successfully
            assert!(db.execute(&statement).is_ok());
        }
    }
}
```

## Coverage Goals

The test files aim to provide:

✅ **Positive coverage**: Every valid SQL feature should have at least one test case
✅ **Negative coverage**: Every error path should have at least one triggering test case
✅ **Combination coverage**: Common feature combinations should be tested
✅ **Edge case coverage**: Boundary conditions and unusual inputs should be tested
✅ **Security coverage**: Known attack patterns should be tested

## Maintenance

When adding new SQL features to EpilogLite:

1.	Add positive test cases demonstrating the feature
2.	Add negative test cases for common misuse
3.	Update the README.md files in each directory
4.	Update this summary document

## References

-	See `sql_positive/README.md` for detailed positive test documentation
-	See `sql_negative/README.md` for detailed negative test documentation
-	See existing test files in `tests/` for integration test examples
-	See `docs/design/sql_syntax/` for SQL syntax specifications
