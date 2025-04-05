ALTER TABLE schema_name.table_name RENAME TO new_table_name;
ALTER TABLE table_name RENAME TO new_table_name;

ALTER TABLE table_name RENAME COLUMN column_name TO new_column_name;
ALTER TABLE table_name RENAME column_name TO new_column_name;

ALTER TABLE table_name ADD COLUMN column_name;
ALTER TABLE table_name ADD column_name;

ALTER TABLE table_name ADD column_name INTEGER CONSTRAINT constraint_name INTEGER PRIMARY KEY ASC ON CONFLICT ROLLBACK AUTOINCREMENT;

ALTER TABLE table_name ADD column_name INTEGER;

ALTER TABLE table_name ADD column_name PRIMARY KEY;
ALTER TABLE table_name ADD column_name PRIMARY KEY DESC;

ALTER TABLE table_name ADD column_name PRIMARY KEY ON CONFLICT ABORT;
ALTER TABLE table_name ADD column_name PRIMARY KEY ON CONFLICT FAIL;
ALTER TABLE table_name ADD column_name PRIMARY KEY ON CONFLICT IGNORE;
ALTER TABLE table_name ADD column_name PRIMARY KEY ON CONFLICT REPLACE;

ALTER TABLE table_name ADD column_name NOT NULL;
ALTER TABLE table_name ADD column_name UNIQUE;
ALTER TABLE table_name ADD column_name CHECK (true=true);
ALTER TABLE table_name ADD column_name DEFAULT (true);
ALTER TABLE table_name ADD column_name DEFAULT 'value';
ALTER TABLE table_name ADD column_name DEFAULT +1;

ALTER TABLE table_name ADD column_name COLLATION BINARY;
ALTER TABLE table_name ADD column_name COLLATION NOCASE;
ALTER TABLE table_name ADD column_name COLLATION RTRIM;

ALTER TABLE table_name ADD column_name GENERATED ALWAYS AS (true) STORED;
ALTER TABLE table_name ADD column_name AS (true) VIRTUAL;
ALTER TABLE table_name ADD column_name AS (true);

ALTER TABLE table_name ADD column_name REFERENCES foreign_table (fk_column_1, fk_column_2) ON DELETE CASCADE ON UPDATE RESTRICT MATCH match_name DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE table_name ADD column_name REFERENCES foreign_table ON DELETE SET NULL ON UPDATE SET DEFAULT MATCH NOT DEFERRABLE INITIALLY IMMEDIATE;
ALTER TABLE table_name ADD column_name REFERENCES foreign_table ON DELETE NO ACTION;

ALTER TABLE table_name DROP COLUMN column_name;
ALTER TABLE table_name DROP column_name;
