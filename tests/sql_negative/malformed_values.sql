-- Unclosed string literal
INSERT INTO users VALUES (1, 'John)

-- Mismatched quotes
INSERT INTO users VALUES (1, 'John")

-- Missing comma between values
INSERT INTO users VALUES (1 2 3)

-- Extra commas in values
INSERT INTO users VALUES (1,, 2, 3)

-- Missing parentheses in VALUES
INSERT INTO users VALUES 1, 2, 3

-- Empty values
INSERT INTO users VALUES ()

-- Missing VALUES keyword
INSERT INTO users (id, name) (1, 'John')

-- Multiple VALUES clauses (invalid)
INSERT INTO users VALUES (1) VALUES (2)

-- Invalid escape sequences
INSERT INTO users VALUES (1, 'John\xyz')

-- Unterminated value list
INSERT INTO users VALUES (1, 2,
