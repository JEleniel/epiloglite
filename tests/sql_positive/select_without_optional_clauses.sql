-- SELECT without WHERE, ORDER BY, GROUP BY - most basic form
SELECT * FROM users

-- SELECT specific columns without any optional clauses
SELECT id, name FROM users

-- SELECT with aggregate but no GROUP BY
SELECT COUNT(*) FROM users

-- SELECT with JOIN but no WHERE or ORDER BY
SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id
