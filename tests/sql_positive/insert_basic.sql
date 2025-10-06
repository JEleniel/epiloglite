-- INSERT with VALUES (no column list)
INSERT INTO users VALUES (1, 'John', 'john@example.com', 25)

-- INSERT with column specification
INSERT INTO users (id, name, email, age) VALUES (2, 'Jane', 'jane@example.com', 30)

-- INSERT with partial columns
INSERT INTO users (id, name) VALUES (3, 'Bob')

-- INSERT with different data types
INSERT INTO products (id, name, price, available) VALUES (1, 'Widget', 19.99, 1)

-- INSERT with integer values
INSERT INTO counters VALUES (1, 100, 200)

-- INSERT with text values
INSERT INTO messages VALUES (1, 'Hello World')

-- INSERT with boolean-like values
INSERT INTO flags (id, enabled) VALUES (1, 1)
