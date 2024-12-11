---
characters: [",", ";", "="]
expressions: [Column Name List, Common Table Expression, Expression, Join Clause, Qualified Table Name, Returning Clause, Subquery]
identifiers: [Column Name, Table Name]
keywords: [ABORT, FAIL, FROM, IGNORE, OR, RECURSIVE, REPLACE, ROLLBACK, SET, UPDATE, WHERE, WITH]
title: UPDATE
---

# UPDATE

```mermaid
graph TB
	st(("°"))
	semi(((";")))
	st --> WITH
	st --> UPDATE
	
	WITH --> RECURSIVE
	WITH --> common_table_expression>Common Table Expression]
	RECURSIVE --> common_table_expression
	common_table_expression -->|#quot;,#quot;| common_table_expression
	common_table_expression --> UPDATE
	
	UPDATE --> qualified_table_name>Qualified Table Name]
	UPDATE --> OR
	OR --> ABORT
	OR --> FAIL
	OR --> IGNORE
	OR --> REPLACE
	OR --> ROLLBACK
	ABORT --> qualified_table_name
	FAIL --> qualified_table_name
	IGNORE --> qualified_table_name
	REPLACE --> qualified_table_name
	ROLLBACK --> qualified_table_name
	qualified_table_name --> SET
	
	SET --> column_name([Column Name])
	SET --> column_name_list>Column Name List]
	column_name -->|#quot;=#quot;| column_expression>Expression]
	column_name_list --> |=| column_expression
	column_expression -->|#quot;,#quot;| column_name
	column_expression -->|#quot;,#quot;| column_name_list
	column_expression --> FROM
	column_expression --> WHERE
	column_expression --> returning_clause>Returning Clause]
	column_expression --> semi
	
	FROM --> schema_name([Schema Name])
	FROM --> table_name([Table Name])
	FROM --> subquery>Subquery]
	FROM --> join_clause>Join Clause]
	FROM --> returning_clause
	
	schema_name -->|#quot;.#quot;| table_name
	table_name -->|#quot;,#quot;| schema_name
	table_name -->|#quot;,#quot;| table_name
	table_name -->|#quot;,#quot;| subquery
	table_name --> WHERE
	table_name --> returning_clause
	table_name --> semi
	
	subquery -->|#quot;,#quot;| schema_name
	subquery -->|#quot;,#quot;| table_name
	subquery -->|#quot;,#quot;| subquery
	subquery --> WHERE
	subquery --> returning_clause
	subquery --> semi
	
	join_clause --> WHERE
	join_clause --> semi
	
	WHERE --> where_expression>Expression]
	where_expression --> returning_clause
	where_expression --> semi
	
	returning_clause --> semi
```
