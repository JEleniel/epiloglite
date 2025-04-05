use crate::parser::{clauses::TableDefClause, expression::Expression, literals::Level2Name};

use super::{SqlProgram, select::SelectStatment};

pub struct CreateStatement {
    unique: bool,
    if_not_exists: bool,
    create_type: CreateType,
}

pub enum CreateType {
    Index(CreateIndexStatement),
    Table(CreateTableStatement),
    Trigger(CreateTriggerStatement),
    View(CreateViewStatement),
    VirtualTable(CreateVirtualTableStatement),
}

pub struct CreateIndexStatement {
    index_name: Level2Name,
    table_name: String,
    column_list: Vec<String>,
    where_expr: Option<Expression>,
}

pub struct CreateTableStatement {
    temporary: bool,
    table_name: Level2Name,
    create_table_type: CreateTableType,
}

pub enum CreateTableType {
    Select(SelectStatment),
    TableDef(TableDefClause),
}

pub struct CreateTriggerStatement {
    temporary: bool,
    if_not_exists: bool,
    trigger_name: Level2Name,
    trigger_type: TriggerType,
    triggering_action: TriggeringAction,
    table_name: String,
    for_each_row: bool,
    when_clause: Option<Expression>,
    program: SqlProgram,
}

pub enum TriggerType {
    Normal,
    Before,
    After,
    InsteadOf,
}

pub enum TriggeringAction {
    Delete,
    Insert,
    Update(Option<Vec<String>>),
}

pub struct CreateViewStatement {
    temporary: bool,
    if_not_exists: bool,
    view_name: Level2Name,
    column_names: Option<Vec<String>>,
    select_statement: SelectStatment,
}

pub struct CreateVirtualTableStatement {
    if_not_exists: bool,
    table_name: Level2Name,
    module_name: String,
    args: Option<Vec<Expression>>,
}
