-- SAVEPOINT creation
SAVEPOINT sp1

-- SAVEPOINT with different name
SAVEPOINT my_savepoint

-- RELEASE savepoint
RELEASE sp1

-- RELEASE SAVEPOINT (alternative syntax)
RELEASE SAVEPOINT my_savepoint

-- ROLLBACK TO savepoint
ROLLBACK TO sp1

-- ROLLBACK TO SAVEPOINT (alternative syntax)
ROLLBACK TO SAVEPOINT my_savepoint
