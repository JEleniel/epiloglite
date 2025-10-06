-- Duplicate PRIMARY KEY constraint
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT PRIMARY KEY)

-- Multiple PRIMARY KEY declarations
CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT PRIMARY KEY)

-- Invalid constraint syntax
CREATE TABLE users (id INTEGER INVALID_CONSTRAINT)

-- Constraint without column
CREATE TABLE users (PRIMARY KEY, name TEXT)

-- Multiple table constraints (might be invalid in some contexts)
CREATE TABLE users (id INTEGER, name TEXT, PRIMARY KEY (id), PRIMARY KEY (name))

-- Invalid constraint order
CREATE TABLE users (PRIMARY KEY id INTEGER)

-- Constraint on wrong element
CREATE TABLE users (id INTEGER), PRIMARY KEY

-- Empty constraint list
CREATE TABLE users (id INTEGER ())

-- Invalid UNIQUE syntax
CREATE TABLE users (id INTEGER UNIQUE UNIQUE)
