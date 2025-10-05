-- Empty statement
 

-- Only whitespace
   	

-- Only comments (if supported)
-- This is just a comment

-- Very long identifier (might exceed limits)
SELECT aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa FROM users

-- Very deeply nested parentheses
SELECT * FROM users WHERE (((((((((((((id = 1)))))))))))))))

-- Special Unicode characters
SELECT * FROM users WHERE name = '你好世界'

-- Zero-width characters
SELECT * FROM users​ WHERE id = 1

-- Multiple semicolons
SELECT * FROM users;;;;;;

-- Statement with only punctuation
;;;,,,((()))

-- Negative numbers in wrong context
CREATE TABLE users (-1 INTEGER)

-- Extremely large numbers
SELECT * FROM users WHERE id = 999999999999999999999999999999999999999999999999

-- Division by zero attempt
SELECT * FROM users WHERE 1/0 = 1

-- Invalid escape sequences
SELECT * FROM users WHERE name = '\x\y\z'

-- Tab and newline characters
SELECT	*	FROM	users
WHERE
id
=
1

-- Mixed case keywords in weird ways
SeLeCt * FrOm UsErS

-- Duplicate table aliases
SELECT * FROM users AS u, orders AS u

-- Self-referential operations
DELETE FROM users WHERE users = users

-- Circular references
SELECT * FROM users WHERE id = id
