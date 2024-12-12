---
characters: [",", ";", ".", "(", ")"]
expressions: [Binary Operator, Bind Value, Expression, Filter Clause, Function Arguments, Literal Value, Over Clause, Raise Function, Table Function, Unary Operator]
identifiers: [Collation Name, Column Name, Function Name, Schema Name, Table Name, Type Name]
keywords: [AND, AS, BETWEEN, CASE, CAST, COLLATE, DISTINCT, ELSE, END, ESCAPE, EXISTS, FROM, GLOB, IN, IS, ISNULL, LIKE, MATCH, NOT, NOTNULL, NULL, REGEXP, THEN, WHEN]
statements: [SELECT]
title: Expression
---

# Expression

```mermaid
graph LR
	st(( ))
	stop(( ))
	
	st --> literal>Literal Value]
	literal --> stop

	st --> bind>Bind Value]
	bind --> stop

	st --> schema_name([Schema Name])
	st --> table_name([Table Name])
	st --> column_name([Column Name])
	schema_name -->|#quot;.#quot;| table_name
	table_name -->|#quot;.#quot;| column_name
	column_name --> stop

	st --> unary>Unary Operator]
	unary --> unary_expression>Expression]
	unary_expression --> stop

	st --> expr_l>Expression]
	expr_l --> binary>Binary Operator]
	binary --> bin_expr_r>Expression]
	bin_expr_r --> stop

	st --> function_name([Function Name])
	function_name -->|"#quot;(#quot;"| function_arguments>Function Arguments]
	function_arguments -->|"#quot;)#quot;"| filter_clause>Filter Clause]
	function_arguments -->|"#quot;)#quot;"| stop
	filter_clause --> over_clause>Over Clause]
	filter_clause --> stop
	over_clause --> stop

	st -->|"#quot;(#quot;"| expression>Expression]
	expression -->|"#quot;)#quot;"| stop

	st --> CAST
	CAST -->|"#quot;(#quot;"| cast_expression>Expression]
	cast_expression --> AS
	AS --> type_name([Type Name])
	type_name -->|"#quot;)#quot;"| stop

	expr_l --> COLLATE
	COLLATE --> collate_name([Collation Name])
	collate_name --> stop

	expr_l --> NOT
	expr_l --> j0((+))
	NOT --> j0
	j0 --> LIKE
	j0 --> GLOB
	j0 --> REGEXP
	j0 --> MATCH
	LIKE --> like_expr>Expression]
	like_expr --> ESCAPE
	like_expr --> stop
	ESCAPE --> escape_expr>Expression]
	escape_expr --> stop
	GLOB --> comp_expr_r>Expression]
	REGEXP --> comp_expr_r
	MATCH --> comp_expr_r
	comp_expr_r --> stop

	expr_l --> ISNULL
	expr_l --> NOTNULL
	NOT --> NULL
	ISNULL --> stop
	NOTNULL --> stop
	NULL --> stop

	expr_l --> IS
	IS --> distinct_clause[DISTINCT FROM]
	IS --> distinct_expr_r>Expression]
	NOT --> distinct_clause
	NOT --> distinct_expr_r
	distinct_clause --> distinct_expr_r
	distinct_expr_r --> stop

	expr_l --> NOT
	expr_l --> BETWEEN
	NOT --> BETWEEN
	BETWEEN --> bet_expr_l>Expression]
	bet_expr_l --> AND
	AND --> bet_expr_r>Expression]
	bet_expr_r --> stop

	expr_l --> IN
	NOT --> IN
	IN --> in_lparen("(")
	IN --> in_schema_name([Schema Name])
	IN --> in_table_name([Table Name])
	IN --> table_function>Table Function]
	in_lparen --> in_rparen(")")
	in_rparen --> stop
	in_lparen --> in_select_statement{{Select Statement}}
	in_lparen --> in_expr>Expression]
	in_select_statement -->|#quot;,#quot;| in_select_statement
	in_select_statement -->|#quot;,#quot;| in_expr
	in_select_statement --> in_rparen
	in_expr -->|#quot;,#quot;| in_select_statement
	in_expr --> in_rparen
	in_expr -->|#quot;,#quot;| in_expr
	in_schema_name -->|#quot;.#quot;| in_table_name
	in_schema_name -->|#quot;.#quot;| table_function
	in_table_name --> stop
	table_function -->|"#quot;(#quot;"| in_expr2>Expression]
	in_expr2 -->|#quot;,#quot;| in_expr2
	in_expr2 -->|"#quot;)#quot;"| stop

	st --> EXISTS
	st -->|"#quot;(#quot;"| exists_select_statement{{Select Statement}}
	NOT --> EXISTS
	EXISTS --> exists_select_statement
	exists_select_statement -->|"#quot;)#quot;"| stop

	st --> CASE
	CASE --> case_expr>Case Expression]
	CASE --> WHEN
	case_expr --> WHEN
	WHEN --> when_expr>Expression]
	when_expr --> THEN
	THEN --> then_expr>Expression]
	then_expr -->|#quot;,#quot;| WHEN
	then_expr --> ELSE
	then_expr --> END
	ELSE --> else_expr>Expression]
	else_expr --> END
	END --> stop

	st --> raise>Raise Function]
	raise --> stop
```

## Used by

```dataview
TABLE WITHOUT ID
	split(file.path,"/")[length(split(file.path,"/"))-2] as Type,
	file.link AS Element
FROM "ba-Projects/EpilogLite/sql_syntax" 
WHERE contains(expressions, this.file.name)
```
