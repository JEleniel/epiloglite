---
regex: '"//[^\n]*"'
title: Single Line Comment
---

# Single Line Comment

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> open["//"]
	open --> not_nl[/"Not newline"\]
	not_nl --> not_nl
	not_nl --> close["newline or EOF"]
	close --> stop
```
