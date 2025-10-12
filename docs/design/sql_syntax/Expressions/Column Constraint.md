---
characters: ["(", ")"]
expressions: [Conflict Clause, Expression, Foreign Key  Clause, Literal Value, Signed Number]
identifiers: [Collation Name, Constraint Name]
keywords: [ALWAYS, ASC, AUTOINCREMENT, CHECK, COLLATE, CONSTRAINT, DESC, GENERATED, NOT, NULL, STORED, UNIQUE, VIRTUAL]
title: Column Constraint
---

# Column Constraint

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> CONSTRAINT
	st --> j0((+))

	CONSTRAINT --> constraint_name([Constraint Name])

	constraint_name --> j0

	j0 --> primary[PRIMARY KEY]
	j0 --> not_null[NOT NULL]
	j0 --> UNIQUE
	j0 --> CHECK
	j0 --> DEFAULT
	j0 --> COLLATE
	j0 --> foreign_clause>Foreign Key Clause]
	j0 --> generated[GENERATED ALWAYS]
	j0 --> AS

	primary --> conflict_clause>Conflict Clause]
	primary --> ASC
	primary --> DESC

	ASC --> conflict_clause
	DESC --> conflict_clause

	conflict_clause --> AUTOINCREMENT
	conflict_clause --> stop

	AUTOINCREMENT --> stop

	not_null --> conflict_clause2>Conflict Clause]

	conflict_clause2 --> stop

	UNIQUE --> conflict_clause2

	CHECK -->|"#quot;(#quot;"| expression>Expression]
	expression -->|"#quot;)#quot;"| stop

	DEFAULT --> expression
	DEFAULT --> literal>Literal Value]
	DEFAULT --> signed_number>Signed Number]

	literal --> stop

	signed_number --> stop

	COLLATE --> collation_name([Collation Name])

	collation_name --> stop

	foreign_clause --> stop

	generated --> AS
	AS -->|"#quot;(#quot;"| a_expr>Expression]

	a_expr -->|"#quot;)#quot;"| STORED
	a_expr -->|"#quot;)#quot;"| VIRTUAL
	a_expr -->|"#quot;)#quot;"| stop 

	STORED --> stop
	VIRTUAL --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                                 |
| ----------- | ------------------------------------------------------- |
| Expressions | [Expressions: Column Definition](<Column%20Definition>) |
<!-- SerializedQuery END -->
