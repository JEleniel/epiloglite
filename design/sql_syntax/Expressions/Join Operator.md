---
characters: [","]
keywords: [CROSS, FULL, INNER, JOIN, LEFT, NATURAL, OUTER, RIGHT]
title: Join Operator
---

# Join Operator

```mermaid
graph TB
	st(( ))
	stop(( ))

	st -->|#quot;,#quot;| stop
	st --> NATURAL
	st --> CROSS
	st --> j0((+))
	st --> j1
		
	JOIN --> stop

	NATURAL --> j0

	j0 --> LEFT
	j0 --> RIGHT
	j0 --> FULL
	j0 --> INNER

	LEFT --> OUTER
	LEFT --> j1((+))

	RIGHT --> OUTER
	RIGHT --> j1

	FULL --> OUTER
	FULL --> j1

	OUTER --> j1

	INNER --> j1

	CROSS --> j1

	j1 --> JOIN
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                     |
| ----------- | ------------------------------------------- |
| Expressions | [Expressions: Join Clause](<Join%20Clause>) |
<!-- SerializedQuery END -->
