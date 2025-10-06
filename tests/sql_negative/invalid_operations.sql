-- Random gibberish
BLAH BLAH BLAH

-- Invalid statement type
INVALID STATEMENT

-- Mixed up statement
DELETE UPDATE INSERT

-- CREATE without TABLE
CREATE users (id INTEGER)

-- Table without CREATE
TABLE users (id INTEGER)

-- SELECT without table
SELECT id, name

-- UPDATE without table
UPDATE SET age = 25

-- INSERT without table
INSERT VALUES (1, 2, 3)

-- Incomplete JOIN
SELECT * FROM users JOIN

-- JOIN without ON condition (when required)
SELECT * FROM users INNER JOIN orders

-- Invalid aggregate function
SELECT INVALID(id) FROM users

-- Aggregate without column
SELECT COUNT() FROM users

-- ORDER BY without column
SELECT * FROM users ORDER BY

-- GROUP BY without column
SELECT * FROM users GROUP BY

-- WHERE without condition
SELECT * FROM users WHERE

-- SET without assignment
UPDATE users SET WHERE id = 1

-- Multiple table names without JOIN
SELECT * FROM users orders

-- Invalid comparison operator
SELECT * FROM users WHERE id <> 1

-- Missing comparison value
SELECT * FROM users WHERE id =

-- Invalid JOIN type
SELECT * FROM users OUTER JOIN orders ON users.id = orders.user_id
