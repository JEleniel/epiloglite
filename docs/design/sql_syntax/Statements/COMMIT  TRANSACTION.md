---
characters: [";"]
keywords: [COMMIT, TRANSACTION]
linter-yaml-title-alias: COMMIT TRANSACTION
title: COMMIT TRANSACTION
---

# COMMIT TRANSACTION

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
	st --> COMMIT
	COMMIT --> TRANSACTION
	TRANSACTION --> semi
```
