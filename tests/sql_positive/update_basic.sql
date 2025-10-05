-- UPDATE without WHERE (updates all rows)
UPDATE users SET age = 25

-- UPDATE with WHERE clause
UPDATE users SET age = 30 WHERE id = 1

-- UPDATE multiple columns
UPDATE users SET name = 'John Doe', age = 35 WHERE id = 1

-- UPDATE with complex WHERE
UPDATE users SET email = 'newemail@example.com' WHERE age > 18

-- UPDATE with string value
UPDATE products SET name = 'New Widget' WHERE id = 1

-- UPDATE with numeric value
UPDATE employees SET salary = 75000 WHERE id = 5
