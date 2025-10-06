-- Complex SELECT with all optional clauses
SELECT department, COUNT(*), AVG(salary) FROM employees WHERE salary > 50000 GROUP BY department ORDER BY department

-- Multi-table JOIN with WHERE and ORDER BY
SELECT users.name, orders.total, products.name FROM users INNER JOIN orders ON users.id = orders.user_id INNER JOIN products ON orders.product_id = products.id WHERE orders.total > 100 ORDER BY orders.total DESC

-- Aggregate with GROUP BY and WHERE
SELECT department, SUM(salary), COUNT(*) FROM employees WHERE active = 1 GROUP BY department ORDER BY SUM(salary) DESC

-- LEFT JOIN with aggregates
SELECT users.name, COUNT(*) FROM users LEFT JOIN orders ON users.id = orders.user_id GROUP BY users.name

-- Multiple aggregates with complex conditions
SELECT MIN(age), MAX(age), AVG(age), COUNT(*) FROM users WHERE age >= 18 ORDER BY COUNT(*)

-- CROSS JOIN with WHERE filter
SELECT * FROM categories CROSS JOIN products WHERE products.price > 10

-- Complex WHERE conditions (would need proper expression parsing)
SELECT * FROM users WHERE age > 18 ORDER BY age DESC
