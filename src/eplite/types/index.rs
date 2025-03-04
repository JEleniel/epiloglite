use super::orderby::OrderBy;

pub struct Index {
    id: u64,
    constraints: Vec<Constraint>,
    order_by: Vec<OrderBy>,
}

pub struct Constraint {
    column: u32,
    operator: char,
    useable: bool,
}
