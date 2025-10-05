-- SELECT with COUNT(*)
SELECT COUNT(*) FROM users

-- SELECT with COUNT(column)
SELECT COUNT(id) FROM users

-- SELECT with SUM
SELECT SUM(salary) FROM employees

-- SELECT with AVG
SELECT AVG(age) FROM users

-- SELECT with MIN
SELECT MIN(age) FROM users

-- SELECT with MAX
SELECT MAX(salary) FROM employees

-- SELECT with multiple aggregates
SELECT COUNT(*), AVG(salary), MAX(salary) FROM employees
