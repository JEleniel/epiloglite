-- SQL injection with semicolon
SELECT * FROM users WHERE id = 1; DROP TABLE users;

-- SQL injection with comment
SELECT * FROM users WHERE name = 'admin'--

-- SQL injection with UNION
SELECT * FROM users WHERE id = 1 UNION SELECT * FROM passwords

-- Tautology injection
SELECT * FROM users WHERE 1=1 OR '1'='1'

-- Stacked queries
INSERT INTO users VALUES (1, 'test'); DELETE FROM users;

-- Command injection attempt
SELECT * FROM users WHERE name = ''; EXEC('cmd.exe');

-- Nested subquery injection
SELECT * FROM users WHERE id = (SELECT id FROM admin)

-- Time-based blind injection
SELECT * FROM users WHERE id = 1; WAITFOR DELAY '00:00:05'

-- Boolean-based blind injection
SELECT * FROM users WHERE id = 1 AND 1=1

-- Hex encoding injection attempt
SELECT * FROM users WHERE name = 0x61646D696E

-- Unicode injection
SELECT * FROM users WHERE name = '\u0061dmin'

-- Null byte injection
SELECT * FROM users WHERE name = 'admin\0'

-- Comment-based injection
SELECT * FROM users WHERE id = 1 /* malicious comment */ AND 1=1

-- Multiple statement injection
BEGIN; DROP TABLE users; COMMIT;
