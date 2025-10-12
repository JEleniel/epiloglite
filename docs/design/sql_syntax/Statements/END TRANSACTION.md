---
aliases: [COMMIT or END TRANSACTION]
characters: [";"]
keywords: [COMMIT, END, TRANSACTION]
linter-yaml-title-alias: COMMIT or END TRANSACTION
title: COMMIT or END TRANSACTION
---

# COMMIT or END TRANSACTION

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
	
	st --> END
	END --> TRANSACTION
	TRANSACTION --> semi
```
