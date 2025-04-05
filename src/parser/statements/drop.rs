use crate::parser::literals::Level2Name;

pub struct DropStatement {
    if_exists: bool,
    drop_type: DropType,
}

pub enum DropType {
    Index(Level2Name),
    Table(Level2Name),
    Trigger(Level2Name),
    View(Level2Name),
}
