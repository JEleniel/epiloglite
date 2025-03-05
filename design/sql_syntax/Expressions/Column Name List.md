---
title: Column Name List
---

# Column Name List

```mermaid
graph TB
	st(( ))
	stop(( ))

	st -->|"#quot;(#quot;"| column_name([Column Name])
	column_name -->|#quot;,#quot;| column_name
	column_name -->|"#quot;)#quot;"| stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                         |
| ----------- | ----------------------------------------------- |
| Expressions | [Expressions: Upsert Clause](<Upsert%20Clause>) |
| Statements  | [Statements: UPDATE](<UPDATE>)                  |
<!-- SerializedQuery END -->
