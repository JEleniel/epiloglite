-- Missing table name
SELECT * FROM

-- Missing FROM keyword
SELECT * users

-- Incomplete statement
SELECT

-- Just a keyword
CREATE

-- Missing closing parenthesis
CREATE TABLE users (id INTEGER

-- Missing opening parenthesis
CREATE TABLE users id INTEGER)

-- Extra closing parenthesis
CREATE TABLE users (id INTEGER))

-- Mismatched parentheses
SELECT * FROM users WHERE (age > 18

-- Extra tokens after valid statement
SELECT * FROM users EXTRA TOKENS

-- Missing column list in CREATE TABLE
CREATE TABLE users ()

-- Missing VALUES keyword in INSERT
INSERT INTO users (1, 2, 3)

-- Missing INTO keyword in INSERT
INSERT users VALUES (1, 2, 3)

-- Missing WHERE expression
SELECT * FROM users WHERE

-- Missing SET clause in UPDATE
UPDATE users WHERE id = 1

-- Missing table name in UPDATE
UPDATE SET age = 25

-- Missing table name in DELETE
DELETE FROM WHERE id = 1

-- Invalid punctuation
SELECT * FFROM users

-- Duplicate keywords
SELECT SELECT * FROM users

-- Wrong keyword order
FROM users SELECT *

-- Missing commas in column list
SELECT id name age FROM users

-- Extra commas
SELECT id,, name FROM users

-- Missing column name
SELECT , name FROM users

-- Trailing comma
SELECT id, name, FROM users
