use crate::parser::expression::Expression;

pub struct AttachStatement {
    expr: Expression,
    schema_name: String,
}

pub struct DetachStatement {
    schema_name: String,
}
