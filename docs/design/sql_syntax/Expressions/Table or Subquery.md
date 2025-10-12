---
characters: [",", ".", "(", ")"]
expressions: [Expression, Join Clause, Table or Subquery]
identifiers: [Index Name, Schema Name, Table Alias, Table Function Name, Table Name]
keywords: [AS, BY, INDEXED, NOT]
statements: [SELECT STATEMENT]
title: Table or Subquery
---

# Table or Subquery

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> schema_name([Schema Name])
	st --> table_name([Table Name])
	st --> table_function_name([Table Function Name])
	st -->|"#quot;(#quot;"| select_statement{{Select Statement}}
	st -->|"#quot;(#quot;"| table_or_subquery>Table or Subquery]
	st -->|"#quot;(#quot;"| join_clause>Join Clause]

	schema_name -->|#quot;.#quot;| table_name
	schema_name -->|#quot;.#quot;| table_function_name

	table_name --> AS
	table_name --> table_alias([Table Alias])
	table_name --> indexed[INDEXED BY]
	table_name --> not_indexed[NOT INDEXED]
	table_name --> stop
	
	AS --> table_alias

	table_alias --> indexed
	table_alias --> not_indexed
	table_alias --> stop

	indexed --> index_name([Index Name])

	index_name --> stop

	not_indexed --> stop

	table_function_name -->|"#quot;(#quot;"| expression>Expression]

	expression -->|#quot;,#quot;|expression
	expression --> |"#quot;)#quot;"| stop
	expression --> j0((+))

	select_statement -->|"#quot;)#quot;"| j0
	select_statement -->|"#quot;)#quot;"| stop

	table_or_subquery -->|#quot;,#quot;| table_or_subquery
	table_or_subquery -->|"#quot;)#quot;"| stop

	join_clause -->|"#quot;)#quot;"| stop

	j0 -->as2[AS]
	j0 --> table_alias2([Table Alias])

	as2 --> table_alias2

	table_alias2 --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                                   |
| ----------- | --------------------------------------------------------- |
| Expressions | [Expressions: Table or Subquery](<Table%20or%20Subquery>) |

<!-- SerializedQuery END -->
