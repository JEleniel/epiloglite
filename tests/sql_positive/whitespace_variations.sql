-- Multiple spaces between keywords
SELECT  *  FROM  users

-- Tab characters
SELECT	*	FROM	users

-- Newlines between clauses
SELECT *
FROM users
WHERE id = 1
ORDER BY name

-- Mixed whitespace
SELECT   *	FROM
	users  WHERE
	age > 18

-- Minimal whitespace
SELECT*FROM users

-- Leading whitespace
   SELECT * FROM users

-- Trailing whitespace
SELECT * FROM users   

-- No spaces around operators (in WHERE clause)
SELECT * FROM users WHERE id=1
