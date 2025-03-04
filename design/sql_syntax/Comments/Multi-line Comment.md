---
regex: '"/\*(?:[^\*]|\*[^/])*(?:\**/|$)"'
title: Multi-line Comment
---

# Multi-line Comment

```mermaid
graph TB
	st(( ))
	stop(( ))

	st --> open["/*"]
	open --> not_close[/"Not */"\]
	not_close --> not_close
	not_close --> close["*/"]
	close --> stop
```
