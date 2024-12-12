---
keywords: [abs, changes, char, coalesce, concat, concat_ws, format, glob, hex, ifnull, iif, instr, last_insert_rowid, length, like, likelihood, likely, load_extension, lower, ltrim, max, min, nullif, octet_length, printf, quote, random, randomblob, replace, round, rtrim, sign, soundex, sqlite_compileoption_get, sqlite_compileoption_used, sqlite_offset, sqlite_source, sqlite_version, substr, substring, total_changes, trim, typeof, unhex, unicode, unlikely, upper, zeroblob]
title: Function Name
---

# Function Name

```mermaid
graph LR
	start(( ))
	stop(( ))

	st --> abs
	st --> changes
	st --> char
	st --> coalesce
	st --> concat
	st --> concat_ws
	st --> format
	st --> glob
	st --> hex
	st --> ifnull
	st --> iif
	st --> instr
	st --> last_insert_rowid
	st --> length
	st --> like
	st --> likelihood
	st --> likely
	st --> load_extension
	st --> lower
	st --> ltrim
	st --> max
	st --> min
	st --> nullif
	st --> octet_length
	st --> printf
	st --> quote
	st --> random
	st --> randomblob
	st --> replace
	st --> round
	st --> rtrim
	st --> sign
	st --> soundex
	st --> sqlite_compileoption_get
	st --> sqlite_compileoption_used
	st --> sqlite_offset
	st --> sqlite_source
	st --> sqlite_version
	st --> substr
	st --> substring
	st --> total_changes
	st --> trim
	st --> typeof
	st --> unhex
	st --> unicode
	st --> unlikely
	st --> upper
	st --> zeroblob

	abs --> stop
	changes --> stop
	char --> stop
	coalesce --> stop
	concat --> stop
	concat_ws --> stop
	format --> stop
	glob --> stop
	hex --> stop
	ifnull --> stop
	iif --> stop
	instr --> stop
	last_insert_rowid --> stop
	length --> stop
	like --> stop
	likelihood --> stop
	likely --> stop
	load_extension --> stop
	lower --> stop
	ltrim --> stop
	max --> stop
	min --> stop
	nullif --> stop
	octet_length --> stop
	printf --> stop
	quote --> stop
	random --> stop
	randomblob --> stop
	replace --> stop
	round --> stop
	rtrim --> stop
	sign --> stop
	soundex --> stop
	sqlite_compileoption_get --> stop
	sqlite_compileoption_used --> stop
	sqlite_offset --> stop
	sqlite_source --> stop
	sqlite_version --> stop
	substr --> stop
	substring --> stop
	total_changes --> stop
	trim --> stop
	typeof --> stop
	unhex --> stop
	unicode --> stop
	unlikely --> stop
	upper --> stop
	zeroblob --> stop
```

## Used by

```dataview
TABLE WITHOUT ID
	split(file.path,"/")[length(split(file.path,"/"))-2] as Type,
	file.link AS Element
FROM "ba-Projects/EpilogLite/sql_syntax" 
WHERE contains(identifiers, this.file.name)
```
