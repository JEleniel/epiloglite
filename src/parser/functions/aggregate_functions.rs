use crate::parser::{
    clauses::{FilteringClause, OrderingTerm},
    expression::Expression,
};

pub enum AggregateFunction {
    General(AggregateFunctionInvocation),
    All(AggregateFunctionAllInvocation),
}

pub struct AggregateFunctionInvocation {
    pub function: AggregateFunctionName,
    pub distinct: bool,
    pub exprs: Vec<Expression>,
    pub ordering_terms: Option<Vec<OrderingTerm>>,
    pub filter_clause: Option<FilteringClause>,
}

pub enum AggregateFunctionName {
    Average,
    Count,
    GroupConcat(Option<String>),
    Min,
    Max,
    Sum,
    Total,
}

pub struct AggregateFunctionAllInvocation {
    pub function: AggregateFunctionAllName,
    pub filter_clause: Option<FilteringClause>,
}

pub enum AggregateFunctionAllName {
    Count,
}
