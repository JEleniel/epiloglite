---
characters: [",", "(", ")", "="]
expressions: [Column Name List, Expression, Indexed Column]
identifiers: [Column Name]
keywords: [CONFLICT, DO, ON, SET, UPDATE, WHERE]
title: Upsert Clause
---

# Upsert Clause

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> on[ON CONFLICT]

	on --> DO
	on -->|"#quot;(#quot;"| indexed_column>Indexed Column]

	indexed_column -->|#quot;,#quot;| indexed_column
	indexed_column -->|"#quot;)#quot;"| DO
	indexed_column -->|"#quot;)#quot;"| WHERE

	WHERE --> expression>Expression]
	expression --> DO

	DO --> NOTHING
	DO --> update[UPDATE SET]

	NOTHING --> on
	NOTHING --> stop

	update --> column_name([Column Name])
	update --> column_list>Column Name List]
	
	column_name --> |#quot;=#quot;| expr2>Expression]

	column_list -->|#quot;=#quot;| expr2
	
	expr2 --> column_name
	expr2 --> column_list
	expr2 --> where2[WHERE]

	where2 --> w_exp>Expression]

	w_exp --> on
	w_exp --> stop
	
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: INSERT](<INSERT>) |
<!-- SerializedQuery END -->
