# Advanced SQL Features

## Overview

EpilogLite must support advanced SQL query capabilities including WHERE clause filtering, JOIN operations, aggregate functions, sorting, and grouping to enable complex data analysis and retrieval operations.

## User Story

As a database user, I need to perform complex queries with filtering, joins, and aggregations so that I can extract meaningful insights from my data and build sophisticated applications.

## Features

### 1. WHERE Clause Filtering
- Comparison operators (=, !=, <, >, <=, >=)
- Logical operators (AND, OR, NOT)
- Pattern matching (LIKE operator)
- IN and BETWEEN operators
- Expression evaluation

**Acceptance Criteria:**
- All comparison operators work correctly with appropriate types
- Logical operators combine conditions properly
- LIKE operator supports % and _ wildcards
- Complex nested conditions evaluate correctly
- NULL comparison handled with IS NULL / IS NOT NULL

### 2. JOIN Operations
- CROSS JOIN (Cartesian product)
- INNER JOIN with ON conditions
- LEFT JOIN / RIGHT JOIN (planned)
- FULL OUTER JOIN (planned)
- Multiple table joins

**Acceptance Criteria:**
- CROSS JOIN produces correct Cartesian product
- INNER JOIN correctly matches rows based on ON conditions
- Join conditions evaluate efficiently
- Multiple joins in single query work correctly
- Result set contains correct combined columns

### 3. Aggregate Functions
- COUNT(*) and COUNT(column)
- SUM(column)
- AVG(column)
- MIN(column)
- MAX(column)
- Support with GROUP BY

**Acceptance Criteria:**
- COUNT returns correct row counts
- SUM, AVG handle numeric types correctly
- MIN/MAX work with all comparable types
- NULL values handled appropriately in aggregations
- Aggregate functions work with and without GROUP BY

### 4. Sorting and Grouping
- ORDER BY single/multiple columns
- ASC/DESC ordering
- GROUP BY single/multiple columns
- HAVING clause for group filtering
- Proper NULL handling in sorting

**Acceptance Criteria:**
- ORDER BY sorts results correctly
- Multiple sort columns work as expected
- GROUP BY correctly groups rows
- HAVING filters groups appropriately
- NULLs sort according to SQLite behavior

### 5. Subqueries
- Scalar subqueries
- IN/EXISTS subqueries
- Subqueries in WHERE clause
- Correlated subqueries (planned)

**Acceptance Criteria:**
- Scalar subqueries return single values
- IN subqueries filter correctly
- EXISTS checks work properly
- Subquery results integrate into outer query correctly
