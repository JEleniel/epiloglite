---
characters: [",", "(", ")"]
expressions: [Expression, Frame Specification, Ordering Term]
identifiers: [Base Window Name]
keywords: [BY, ORDER, PARTITION]
title: Window Definition
---

# Window Definition

```mermaid
graph TB
	st(( ))
	stop(( ))

	st -->lparen("(") 
	
	lparen --> base_window_name([Base Window Name])
	lparen --> j_part((+))
	j_part --> partition[PARTITION BY]
	lparen --> j_ord((+))
	j_ord --> order[ORDER BY]
	lparen --> j_frame((+))
	j_frame --> frame_spec>Frame Specification]
	lparen --> rparen(")")

	base_window_name --> j_part
	base_window_name --> j_ord
	base_window_name --> j_frame
	base_window_name --> rparen
	
	partition --> p_expr>Expression]
	
	p_expr -->|#quot;,#quot;| p_expr
	p_expr --> j_ord
	p_expr --> j_frame
	p_expr --> rparen
	
	order --> ordering_term>Ordering Term]

	ordering_term -->|#quot;,#quot;| ordering_term
	ordering_term --> j_frame
	ordering_term --> rparen
	
	frame_spec --> rparen

	rparen --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type       | Element                        |
| ---------- | ------------------------------ |
| Statements | [Statements: SELECT](<SELECT>) |
<!-- SerializedQuery END -->
