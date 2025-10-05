-- SELECT with WHERE and ORDER BY
SELECT * FROM users WHERE age > 18 ORDER BY name

-- SELECT with WHERE, GROUP BY, and ORDER BY
SELECT department, COUNT(*) FROM employees WHERE salary > 50000 GROUP BY department ORDER BY department

-- SELECT with JOIN and WHERE
SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE users.age > 18

-- SELECT with aggregates and GROUP BY
SELECT department, AVG(salary) FROM employees GROUP BY department

-- SELECT specific columns with WHERE and ORDER BY
SELECT id, name, email FROM users WHERE age >= 21 ORDER BY name ASC

-- SELECT with JOIN, WHERE, and ORDER BY
SELECT users.name, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY orders.total DESC
