---
characters: [";"]
identifiers: [Save Point Name]
keywords: [SAVEPOINT]
title: SAVEPOINT
---

# SAVEPOINT

```mermaid
---
config:
  layout: elk
---
graph LR
	st(( ))
	semi(;)
	stop(( ))
	semi --> stop
	
	st --> SAVEPOINT
	SAVEPOINT --> savepoint_name([Save Point Name])
	savepoint_name --> semi
```
