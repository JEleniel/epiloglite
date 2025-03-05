---
expressions: [Expression]
identifiers: [Collation Name, Column Name]
keywords: [ASC, COLLATE, DESC]
title: Indexed Column
---

# Indexed Column

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> column_name([Column Name])
	st --> expression>Expression]
	
	column_name --> j0((+))
	expression --> j0
		
	j0 --> j1((+))
	j0 --> COLLATE

	j1 --> ASC
	j1 --> DESC
	j1 --> stop
	
	COLLATE --> collate_name([Collation Name])
	collate_name --> j1
	
	ASC --> stop
	DESC --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                               |
| ----------- | ----------------------------------------------------- |
| Expressions | [Expressions: Upsert Clause](<Upsert%20Clause>)       |
| Expressions | [Expressions: Table Constraint](<Table%20Constraint>) |
| Statements  | [Statements: CREATE](<CREATE>)                        |

<!-- SerializedQuery END -->
