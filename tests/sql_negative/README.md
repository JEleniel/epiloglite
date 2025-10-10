# Negative SQL Test Cases

This directory contains malformed, invalid, and adversarial SQL statements that should fail to parse or execute in EpilogLite.

## Purpose

These test files provide comprehensive coverage of:

-	Invalid SQL syntax
-	Malformed statements
-	Security vulnerabilities (SQL injection attempts)
-	Edge cases and boundary conditions
-	Error handling verification

## Test Files

### Syntax Errors

-	**syntax_errors.sql** - Fundamental syntax violations
   	+	Missing keywords (SELECT, FROM, WHERE, etc.)
   	+	Incomplete statements
   	+	Mismatched parentheses
   	+	Extra tokens
   	+	Wrong keyword order

### Invalid Identifiers

-	**invalid_identifiers.sql** - Identifier-related errors
   	+	Empty identifiers
   	+	Invalid characters in names
   	+	Numeric-only table names
   	+	Reserved keywords used incorrectly
   	+	SQL injection in identifiers

### Type System Errors

-	**type_errors.sql** - Data type violations
   	+	Invalid type names
   	+	Missing types
   	+	Wrong type syntax
   	+	Type mismatches

### Value Errors

-	**malformed_values.sql** - Invalid value literals
   	+	Unclosed strings
   	+	Mismatched quotes
   	+	Missing commas
   	+	Empty value lists
   	+	Invalid escape sequences

### Security Tests

-	**injection_attempts.sql** - SQL injection attack patterns
   	+	Semicolon-based injection
   	+	Comment-based injection
   	+	UNION-based injection
   	+	Tautology attacks
   	+	Stacked queries
   	+	Time-based blind injection
   	+	Boolean-based blind injection

### Operation Errors

-	**invalid_operations.sql** - Invalid SQL operations
   	+	Wrong statement structure
   	+	Missing required clauses
   	+	Invalid function names
   	+	Incomplete operations
   	+	Multiple table names without JOIN

### Numeric Errors

-	**numeric_errors.sql** - Number format violations
   	+	Multiple decimal points
   	+	Invalid numeric characters
   	+	Malformed scientific notation
   	+	Invalid negative numbers

### Operator Errors

-	**operator_errors.sql** - Operator misuse
   	+	Missing operands
   	+	Invalid operators
   	+	Double operators
   	+	Wrong operator placement

### Aggregate Errors

-	**aggregate_errors.sql** - Aggregate function violations
   	+	Missing parentheses
   	+	Empty aggregates
   	+	Invalid aggregate names
   	+	Wrong aggregate usage

### JOIN Errors

-	**join_errors.sql** - JOIN clause violations
   	+	Missing ON clause
   	+	Invalid JOIN types
   	+	Incomplete JOIN syntax
   	+	Missing table names

### Constraint Violations

-	**constraint_violations.sql** - Constraint syntax errors
   	+	Duplicate PRIMARY KEY
   	+	Invalid constraint syntax
   	+	Multiple PRIMARY KEY declarations

### Transaction Errors

-	**transaction_errors.sql** - Transaction control violations
   	+	COMMIT without BEGIN
   	+	Invalid savepoint operations
   	+	Missing savepoint names
   	+	Malformed transaction statements

### Edge Cases

-	**edge_cases.sql** - Boundary conditions and unusual inputs
   	+	Empty statements
   	+	Only whitespace
   	+	Very long identifiers
   	+	Deeply nested parentheses
   	+	Unicode and special characters
   	+	Zero-width characters
   	+	Multiple semicolons

### Buffer Overflow Attempts

-	**buffer_overflow_attempts.sql** - Stress testing with large inputs
   	+	Extremely long statements
   	+	Very long table/column names
   	+	Many columns
   	+	Deep nesting
   	+	Massive number of JOINs

### Quote Escabing Errors

-	**quote_escabing_errors.sql** - String literal violations
   	+	Unescaped quotes
   	+	Mismatched quote types
   	+	Unclosed strings
   	+	Invalid escape sequences

## Usage

These files can be used to:

1.	**Verify error handling** - Ensure invalid SQL is properly rejected
2.	**Security testing** - Confirm SQL injection attacks are blocked
3.	**Robustness testing** - Verify parser handles malformed input gracefully
4.	**Regression testing** - Ensure errors remain errors across versions

## Testing Approach

Each statement in these files should:

-	❌ Fail to parse (syntax error), OR
-	❌ Fail to execute (semantic error), OR
-	❌ Be properly sanitized (injection attempts)

**None of these statements should execute successfully with their intended malicious effect.**

## Coverage

The test cases cover:

-	✅ Syntax errors in all statement types
-	✅ SQL injection attack vectors
-	✅ Buffer overflow attempts
-	✅ Type system violations
-	✅ Constraint violations
-	✅ Operator misuse
-	✅ Aggregate function errors
-	✅ JOIN clause errors
-	✅ Quote and escape handling
-	✅ Edge cases and boundary conditions
-	✅ Transaction control errors
-	✅ Numeric format errors

## Security Considerations

The SQL injection test cases are included to ensure EpilogLite properly handles:

-	Input sanitization
-	Statement separation prevention
-	Comment-based bypasses
-	Union-based data extraction attempts
-	Blind injection techniques

These tests should **never** result in:

-	Execution of unauthorized commands
-	Data disclosure
-	Database corruption
-	Privilege escalation
