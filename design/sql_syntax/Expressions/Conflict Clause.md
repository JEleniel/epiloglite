---
title: Conflict Clause
---

# Conflict Clause

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> on[ON CONFLICT]
	st --> stop

	on --> ROLLBACK
	on --> ABORT
	on --> FAIL
	on --> IGNORE
	on --> REPLACE

	ROLLBACK --> stop

	ABORT --> stop

	FAIL --> stop

	IGNORE --> stop

	REPLACE --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                               |
| ----------- | ----------------------------------------------------- |
| Expressions | [Expressions: Table Constraint](<Table%20Constraint>) |
<!-- SerializedQuery END -->
