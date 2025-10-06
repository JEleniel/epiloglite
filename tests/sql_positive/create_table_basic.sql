-- CREATE TABLE with single column
CREATE TABLE simple (id INTEGER)

-- CREATE TABLE with multiple columns
CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)

-- CREATE TABLE with PRIMARY KEY
CREATE TABLE users_pk (id INTEGER PRIMARY KEY, name TEXT)

-- CREATE TABLE with various types
CREATE TABLE products (id INTEGER, name TEXT, price DOUBLE, quantity INTEGER)

-- CREATE TABLE with TEXT columns
CREATE TABLE messages (id INTEGER, title TEXT, body TEXT)

-- CREATE TABLE with BOOLEAN type
CREATE TABLE flags (id INTEGER, enabled BOOLEAN)

-- CREATE TABLE with BIGINT
CREATE TABLE large_numbers (id INTEGER, value BIGINT)

-- CREATE TABLE with REAL type
CREATE TABLE measurements (id INTEGER, temperature REAL, humidity REAL)

-- CREATE TABLE with BLOB type
CREATE TABLE files (id INTEGER, filename TEXT, data BLOB)
