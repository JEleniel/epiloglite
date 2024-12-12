---
characters: [",", "(", ")", "*"]
expressions: [Expression, Filter Clause, Ordering Term]
identifiers: [Aggregate Function Name]
keywords: [BY, DISTINCT, ORDER]
title: Aggregate
---

# Aggregate

```mermaid
graph TB
	st(( ))
	lparen("(")
	rparen(")")
	stop(( ))
	st --> aggregate_function([Aggregate Function Name])
	aggregate_function --> lparen
	lparen --> DISTINCT
	lparen --> expression>Expression]
	lparen --> ast("*")
	lparen --> rparen
	DISTINCT --> expression
	expression -->|#quot;,#quot;| expression
	expression --> order_by[ORDER BY]
	expression --> rparen
	order_by --> ordering_term>Ordering Term]
	ordering_term -->|#quot;,#quot;| ordering_term
	ordering_term --> rparen
	ast --> rparen
	rparen --> filter_clause>Filter Clause]
	rparen --> stop
	filter_clause --> stop
```

## Used by

```dataview
TABLE WITHOUT ID
	split(file.path,"/")[length(split(file.path,"/"))-2] as Type,
	file.link AS Element
FROM "ba-Projects/EpilogLite/sql_syntax" 
WHERE contains(expressions, this.file.name)
```
