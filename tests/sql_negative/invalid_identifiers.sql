-- Empty identifier
SELECT FROM users

-- Numbers as table names (without quotes)
SELECT * FROM 123

-- Special characters in unquoted identifier
SELECT * FROM users@domain

-- SQL injection in table name
SELECT * FROM users; DROP TABLE users;

-- Spaces in unquoted identifier
SELECT * FROM user table

-- Reserved keywords without proper escaping (might be handled)
SELECT * FROM select

-- Invalid characters in column names
SELECT id$ FROM users

-- Starting with number (unquoted)
SELECT 1column FROM users

-- Empty column list in INSERT
INSERT INTO users () VALUES (1, 2, 3)

-- Null character in identifier (if not handled properly)
SELECT * FROM users\0

-- Control characters
SELECT * FROM users\n\r\t
