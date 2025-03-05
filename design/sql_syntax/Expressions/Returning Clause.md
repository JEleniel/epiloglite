---
characters: [",", "*"]
expressions: [Expression]
identifiers: [Column Alias]
keywords: [AS, RETURNING]
title: Returning Clause
---

# Returning Clause

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> RETURNING

	RETURNING --> j0((+))
	
	j0 --> expression>Expression]
	j0 --> ast(*)
	
	expression --> AS
	expression --> column_alias([Column Alias])
	expression -->|#quot;,#quot;| j0
	expression --> j1((+))
	
	j1 --> stop
	
	AS --> column_alias

	column_alias -->|#quot;,#quot;| j0
	column_alias --> j1
	
	ast -->|#quot;,#quot;| j0
	ast --> j1
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: UPDATE](<UPDATE>) |
| Statements | [Statements: DELETE](<DELETE>) |
<!-- SerializedQuery END -->
