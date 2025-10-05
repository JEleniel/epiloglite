-- Missing right operand
SELECT * FROM users WHERE age >

-- Missing left operand
SELECT * FROM users WHERE > 18

-- Invalid operator
SELECT * FROM users WHERE age <=> 18

-- Double operators
SELECT * FROM users WHERE age >> 18

-- Missing operator
SELECT * FROM users WHERE age 18

-- Assignment in WHERE (might be invalid)
SELECT * FROM users WHERE age := 18

-- Bitwise operators in wrong context
SELECT * FROM users WHERE age & 18

-- Invalid comparison chain
SELECT * FROM users WHERE 10 < age < 20

-- Operator without operands
SELECT * FROM users WHERE =

-- Multiple equals
SELECT * FROM users WHERE age == 18

-- Invalid NOT placement
SELECT * FROM users WHERE age NOT 18

-- Invalid AND/OR
SELECT * FROM users WHERE AND age > 18

-- Trailing operator in WHERE
SELECT * FROM users WHERE age > 18 AND

-- Leading operator in WHERE
SELECT * FROM users WHERE AND age > 18 OR name = 'John'
