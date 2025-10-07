-- Invalid data type in CREATE TABLE
CREATE TABLE users (id INVALID_TYPE)

-- Missing data type
CREATE TABLE users (id)

-- Missing column name but has type
CREATE TABLE users (INTEGER)

-- Invalid type name with special chars
CREATE TABLE users (id INT@GER)

-- Random text as type
CREATE TABLE users (id FOOBAR)

-- Numeric type name
CREATE TABLE users (id 123)

-- Empty type
CREATE TABLE users (id )

-- Reserved keyword as type (that's not valid)
CREATE TABLE users (id SELECT)
