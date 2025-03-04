---
characters: ["(", ")"]
expressions: [Expression]
keywords: [FILTER]
title: Filter Clause
---

# Filter Clause

```mermaid
graph TB
	st(( ))
	stop(( ))
	st --> FILTER
	FILTER -->|"#quot;(#quot;"| expression>Expression]
	expression -->|"#quot;)#quot;"| stop
```

## Used by

```dataview
TABLE WITHOUT ID
	split(file.path,"/")[length(split(file.path,"/"))-2] as Type,
	file.link AS Element
FROM "ba-Projects/EpilogLite/sql_syntax" 
WHERE contains(expressions, this.file.name)
```
