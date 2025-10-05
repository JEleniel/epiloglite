-- SELECT with INNER JOIN
SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id

-- SELECT with LEFT JOIN
SELECT * FROM users LEFT JOIN orders ON users.id = orders.user_id

-- SELECT with RIGHT JOIN
SELECT * FROM users RIGHT JOIN orders ON users.id = orders.user_id

-- SELECT with CROSS JOIN
SELECT * FROM users CROSS JOIN products

-- SELECT with multiple joins
SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN products ON orders.product_id = products.id
