-- JOIN without second table
SELECT * FROM users INNER JOIN

-- JOIN without ON clause
SELECT * FROM users INNER JOIN orders

-- ON without condition
SELECT * FROM users INNER JOIN orders ON

-- Invalid ON condition
SELECT * FROM users INNER JOIN orders ON =

-- JOIN keyword alone
SELECT * FROM users JOIN JOIN orders ON users.id = orders.user_id

-- Multiple ON clauses
SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id ON users.id = orders.user_id

-- Invalid JOIN type
SELECT * FROM users OUTER JOIN orders ON users.id = orders.user_id

-- JOIN with missing table name
SELECT * FROM users INNER JOIN ON id = id

-- Incomplete JOIN type
SELECT * FROM users INNER ON id = id

-- JOIN with invalid syntax
SELECT * FROM users JOIN INNER orders ON users.id = orders.user_id

-- CROSS JOIN with ON clause (might be invalid)
SELECT * FROM users CROSS JOIN orders ON users.id = orders.user_id

-- Duplicate JOIN keywords
SELECT * FROM users INNER INNER JOIN orders ON users.id = orders.user_id

-- JOIN with WHERE instead of ON
SELECT * FROM users INNER JOIN orders WHERE users.id = orders.user_id
