-- CREATE TABLE with PRIMARY KEY constraint
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)

-- CREATE TABLE with multiple constraints
CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT, price DOUBLE)

-- CREATE TABLE with NOT NULL (if supported)
CREATE TABLE required_fields (id INTEGER PRIMARY KEY, name TEXT)

-- CREATE TABLE with complex schema
CREATE TABLE employees (id INTEGER PRIMARY KEY, name TEXT, department TEXT, salary INTEGER, hire_date TEXT)
