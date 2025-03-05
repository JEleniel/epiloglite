---
characters: [",", "(", ")"]
expressions: [Conflict Clause, Expression, Foreign Key Clause, Indexed Column]
identifiers: [Column Name, Constraint Name]
keywords: [CHECK, CONSTRAINT, FOREIGN, KEY, PRIMARY, UNIQUE]
title: Table Constraint
---

# Table Constraint

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> CONSTRAINT
	st --> j0((+))
	j0 --> primary_clause[PRIMARY KEY]
	j0 --> UNIQUE
	j0 --> CHECK
	j0 --> foreign_clause[FOREIGN KEY]

	CONSTRAINT --> constraint_name([Constraint Name])
	constraint_name --> j0

	primary_clause -->|"#quot;(#quot;"| indexed_column>Indexed Column]
	indexed_column -->|#quot;,#quot;| indexed_column
	indexed_column -->|"#quot;)#quot;"| conflict_clause>Conflict Clause]
	conflict_clause --> stop

	UNIQUE -->|"#quot;(#quot;"| indexed_column

	CHECK -->|"#quot;(#quot;"| expression>Expression]
	expression -->|"#quot;)#quot;"| stop

	foreign_clause -->|"#quot;(#quot;"| column_name([Column Name])
	column_name -->|#quot;,#quot;| column_name
	column_name -->|"#quot;)#quot;"| foreign_key_clause>Foreign Key Clause]
	column_name -->|"#quot;)#quot;"| stop

	foreign_key_clause --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: CREATE](<CREATE>) |
<!-- SerializedQuery END -->
