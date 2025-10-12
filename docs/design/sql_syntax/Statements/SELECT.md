---
characters: [",", ";", "(", ")"]
expressions: [Common Table Expression, Compound Operator, Expression, Join Clause, Ordering Term, Result Column, Table or Subquery, Window Definition]
identifiers: [Window Name]
keywords: [ALL, AS, BY, DISTINCT, FROM, GROUP, HAVING, LIMIT, OFFSET, ORDER, RECURSIVE, SELECT, VALUES, WHERE, WINDOW, WITH]
title: SELECT
---

# SELECT

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
	
	st --> WITH
	st --> SELECT
	st --> VALUES
	
	WITH --> RECURSIVE
	WITH --> common_table_expression>Common Table Expression]
	RECURSIVE --> common_table_expression
	
	common_table_expression -->|#quot;,#quot;| common_table_expression 
	common_table_expression --> j0((+))
	
	j0 --> SELECT
	j0 --> VALUES
	
	SELECT --> result_column>Result Column]
	SELECT --> DISTINCT
	SELECT --> ALL
	DISTINCT --> result_column
	ALL --> result_column
	
	result_column -->|#quot;,#quot;| result_column
	result_column --> j1((+))
		
	j1 --> FROM
	j1 --> j2((+))
	
	j2 --> WHERE
	j2 --> j3((+))
	
	j3 --> GROUP
	j3 --> HAVING
	j3 --> j4((+))
	
	j4 --> WINDOW
	j4 --> j5((+))
	
	j5 --> compound_operator>Compound Operator]
	j5 --> order_clause[ORDER BY]
	j5 --> j6((+))
	
	j6 --> LIMIT
	j6 --> semi
	
	FROM --> table_or_subquery>Table or Subquery]
	FROM --> join_clause>Join Clause]


	table_or_subquery -->|#quot;,#quot;| table_or_subquery
	table_or_subquery --> j2

	join_clause --> j2
	
	WHERE --> where_expression>Expression]
	where_expression --> j3
	
	GROUP --> GROUP_BY
	GROUP_BY --> by_expression>Expression]
	by_expression -->|#quot;,#quot;| by_expression
	by_expression --> HAVING
	by_expression --> j4
	
	HAVING --> hav_expression>Expression]
	hav_expression --> j4
	
	WINDOW --> window_name([Window Name])
	window_name --> AS
	AS --> window_definition>Window Definition]
	window_definition -->|#quot;,#quot;| window_name
	window_definition --> j5
	
	VALUES -->|"#quot;(#quot;"| values_expression>Expression]
	values_expression -->|#quot;,#quot;| values_expression
	values_expression -->|"#quot;),(#quot;"| values_expression
	values_expression -->|"#quot;)#quot;"| j5
	
	compound_operator --> j0
	
	order_clause --> ordering_term>Orderng Term]
	ordering_term -->|#quot;,#quot;| ordering_term
	ordering_term --> j6
	
	LIMIT --> limit_expression>Expression]
	limit_expression --> OFFSET
	limit_expression -->|#quot;,#quot;| limit_expression2>Expression]
	limit_expression --> semi
	limit_expression2 --> semi
		
	OFFSET --> offset_expression>expression]
	offset_expression --> semi

```
