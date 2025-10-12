---
expressions: [Signed Literal, Signed Number]
identifiers: [Name]
title: Pragma Value
---

# Pragma Value

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> signed_number>Signed Number]
	st --> name([Name])
	st --> signed_literal>Signed Literal]

	signed_number --> stop
	name --> stop
	signed_literal --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: PRAGMA](<PRAGMA>) |
<!-- SerializedQuery END -->
