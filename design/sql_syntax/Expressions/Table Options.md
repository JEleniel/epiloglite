---
title: Table Options
---

# Table Options

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> j0((+))
	j0 --> without_clause[WITHOUT ROWID]
	j0 --> STRICT

	without_clause --> j1((+))
	STRICT --> j1

	j1 --> |#quot;,#quot;| j0
	j1 --> stop
	
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: CREATE](<CREATE>) |
<!-- SerializedQuery END -->
