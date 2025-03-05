---
characters: ["-", "+"]
expressions: [Numeric Literal]
title: Signed Number
---

# Signed Number

```mermaid
graph TB
	st(( ))
	stop(( ))

	st -->|#quot;+#quot;| numeric_literal>Numeric Literal]
	st -->|#quot;-#quot;| numeric_literal>Numeric Literal]
	st --> numeric_literal

	numneric_literal --> stop
```

## Used by

<!-- QueryToSerialize: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->
<!-- SerializedQuery: TABLE WITHOUT ID split(file.path,"/")[length(split(file.path,"/"))-2] as Type, "[" + split(file.path,"/")[length(split(file.path,"/"))-2] + ": " + file.name + "](<" + replace(file.name," ","%20") + ">)" AS Element FROM "ba-Projects/EpilogLite/sql_syntax" WHERE contains(expressions, this.file.name) -->

| Type        | Element                                       |
| ----------- | --------------------------------------------- |
| Expressions | [Expressions: Pragma Value](<Pragma%20Value>) |
<!-- SerializedQuery END -->
