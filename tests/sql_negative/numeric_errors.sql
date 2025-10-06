-- Invalid numeric format
SELECT * FROM users WHERE age = 12.34.56

-- Number with invalid characters
SELECT * FROM users WHERE age = 12a34

-- Leading zeros that might be invalid
SELECT * FROM users WHERE age = 0000001

-- Negative zero
SELECT * FROM users WHERE age = -0

-- Number with spaces
SELECT * FROM users WHERE age = 1 2 3

-- Invalid scientific notation
SELECT * FROM users WHERE age = 1.23e

-- Double negative
SELECT * FROM users WHERE age = --5

-- Plus sign in wrong place
SELECT * FROM users WHERE age = +

-- Multiple decimal points
INSERT INTO products VALUES (1, 'test', 19..99)

-- Number starting with decimal
SELECT * FROM users WHERE value = .123

-- Number ending with decimal
SELECT * FROM users WHERE value = 123.

-- Hexadecimal without proper prefix
SELECT * FROM users WHERE id = ABCDEF

-- Binary number (might not be supported)
SELECT * FROM users WHERE id = 0b1010
