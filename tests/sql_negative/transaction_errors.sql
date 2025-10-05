-- COMMIT without BEGIN
COMMIT

-- ROLLBACK without BEGIN
ROLLBACK

-- BEGIN without matching COMMIT or ROLLBACK
BEGIN BEGIN

-- Invalid transaction syntax
BEGIN TRANSACTION IMMEDIATELY

-- SAVEPOINT without name
SAVEPOINT

-- RELEASE without savepoint name
RELEASE

-- ROLLBACK TO without savepoint name
ROLLBACK TO

-- Invalid savepoint name
SAVEPOINT 123

-- Release non-existent savepoint (might be runtime error)
RELEASE sp_nonexistent

-- Rollback to non-existent savepoint (might be runtime error)
ROLLBACK TO sp_nonexistent

-- Multiple transaction keywords
BEGIN COMMIT

-- Invalid transaction command
TRANSACTION START

-- Malformed SAVEPOINT
SAVEPOINT sp1 sp2

-- Extra keywords after transaction command
COMMIT NOW
