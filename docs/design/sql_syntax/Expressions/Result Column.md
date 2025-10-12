---
title: Result Column
---

# Result Column

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> expression>Expression]
	st --> ast(*)
	st --> table_name([Table Name])

	expression --> AS
	expression --> column_alias([Column Alias])
	expression --> stop

	AS --> column_alias

	column_alias --> stop

	table_name -->|#quot;.#quot;| ast

	ast --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: SELECT](<SELECT>) |
<!-- SerializedQuery END -->
