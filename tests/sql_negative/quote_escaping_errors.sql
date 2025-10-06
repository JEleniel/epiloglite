-- Unescaped quote in string
SELECT * FROM users WHERE name = 'O'Brien'

-- Mismatched quote types
SELECT * FROM users WHERE name = 'John"

-- Unclosed string
SELECT * FROM users WHERE name = 'John

-- Double closing quotes
SELECT * FROM users WHERE name = 'John''

-- Empty quotes
SELECT * FROM users WHERE name = ''

-- Only quote character
SELECT * FROM users WHERE name = '

-- Backslash without proper escape
SELECT * FROM users WHERE name = 'John\Doe'

-- Mixed quote types in identifier
SELECT * FROM "users' WHERE id = 1

-- Quote in the middle nowhere
SELECT * FROM users WHERE ' id = 1

-- Multiple quote errors
INSERT INTO users VALUES ('test', 'value', 'another')
