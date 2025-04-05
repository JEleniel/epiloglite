use crate::parser::{clauses::ColumnDef, literals::Level2Name};

pub struct AlterTableStatement {
    table_name: Level2Name,
    alter_table_action: AlterTableAction,
}

pub enum AlterTableAction {
    RenameTable(String),
    RenameColumn(String, String),
    AddColumn(ColumnDef),
    DropColumn(String),
}
