---
characters: [",", "(", ")", "*"]
expressions: [Expression]
identifiers: [Function Name]
title: Function
---

# Function

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> function_name([Function Name])
	function_name --> lparen("(")
	lparen --> expression>Expression]
	lparen --> ast(*)
	lparen --> rparen(")")
	expression --> |#quot;,#quot;| expression
	expression --> rparen
	ast --> rparen
	rparen --> stop
```

## Used by

```dataview
TABLE WITHOUT ID
	split(file.path,"/")[length(split(file.path,"/"))-2] as Type,
	file.link AS Element
FROM "ba-Projects/EpilogLite/sql_syntax" 
WHERE contains(expressions, this.file.name)
```
