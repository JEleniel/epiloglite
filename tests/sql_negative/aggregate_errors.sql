-- Aggregate without closing paren
SELECT COUNT(* FROM users

-- Aggregate without opening paren
SELECT COUNT*) FROM users

-- Multiple columns in single-column aggregate
SELECT AVG(age, salary) FROM users

-- Empty aggregate
SELECT SUM() FROM users

-- Aggregate of aggregate (might be invalid without subquery)
SELECT SUM(AVG(salary)) FROM users

-- COUNT without argument and no asterisk
SELECT COUNT FROM users

-- Wrong aggregate syntax
SELECT * COUNT FROM users

-- Aggregate on non-existent function
SELECT MEDIAN(age) FROM users

-- String in numeric aggregate
SELECT SUM('text') FROM users

-- Nested aggregates improperly
SELECT MAX(MIN(age)) FROM users

-- Aggregate in WHERE (might be invalid without subquery)
SELECT * FROM users WHERE COUNT(*) > 5

-- Invalid aggregate name
SELECT SUMM(salary) FROM users

-- Aggregate with wrong arguments
SELECT COUNT(*, id) FROM users
