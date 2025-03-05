---
expressions: [Join Constraint, Join Operator, Table or Subquery]
title: Join Clause
---

# Join Clause

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> table_or_subquery>Table or Subquery]

	table_or_subquery --> join_operator>Join Operator]
	table_or_subquery --> stop

	join_operator --> j_table_or_subquery>Table or Subquery]

	j_table_or_subquery --> join_constraint>Join Constraint]

	join_constraint --> join_operator
	join_constraint --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                                   |
| ----------- | --------------------------------------------------------- |
| Expressions | [Expressions: Table or Subquery](<Table%20or%20Subquery>) |
| Statements  | [Statements: SELECT](<SELECT>)                            |
| Statements  | [Statements: UPDATE](<UPDATE>)                            |
<!-- SerializedQuery END -->
