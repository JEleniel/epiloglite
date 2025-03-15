use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SqlParser;

pub struct SqlProgram {
    pub statements: Vec<SqlStatement>,
}

pub enum SqlStatement {
    Alter(AlterStatement),
    Analyze(AnalyzeStatement),
    Attach(AttachStatement),
    Begin(BeginStatement),
    Commit,
    Create(CreateStatement),
    Delete(DeleteStatement),
    Detach(DetachStatement),
    Drop(DropStatement),
    Explain(Box<SqlStatement>),
    Insert(InsertStatement),
    Pragma(PragmaStatement),
    Reindex(ReindexStatement),
    Release(ReleaseStatement),
    Rollback(RollbackStatement),
    Savepoint(SavepointStatement),
    Select(SelectStatement),
    Update(UpdateStatement),
    Vacuum(VacuumStatement),
}

pub struct AlterStatement {
    table_name: String,
    alter_action: AlterAction,
}

pub enum AlterAction {
    RenameTo(String),
    RenameColumn(String, String),
    AddCoilumn(ColumnDef),
    DropColumn(String),
}

pub struct AnalyzeStatement {
    name: String,
}

pub struct AttachStatement {
    expression: Expression,
    as_name: String,
}

pub enum BeginStatement {
    Deferred,
    Exclusive,
    Immediate,
    Normal,
}

pub enum CreateStatement {
    CreateIndex(CreateIndexStatement),
    CreateTable(CreateTableStatement),
    CreateTrigger(CreateTriggerStatement),
    CreateView(CreateViewStatement),
    CreateVirtualTable(CreateVirtualTableStatement),
}

pub struct CreateIndexStatement {
    unique: bool,
    if_not_exists: bool,
    name: String,
    table_name: String,
    indexed_columns: Vec<IndexedColumn>,
    where_clause: WhereClause,
}

pub struct CreateTableStatement {
    temporary: bool,
    if_not_exists: bool,
    name: String,
    column_defs: Vec<ColumnDef>,
    table_constraints: Vec<TableConstraint>,
    table_options: TableOptions,
}

pub struct CreateTriggerStatement {
    temporary: bool,
    if_not_exits: bool,
    name: String,
    trigger_type: TriggerType,
    trigger_on_action: TriggerOnAction,
    on_name: String,
    for_each_row: bool,
    when_clause: Option<WhenClause>,
    trigger_actions: Vec<TriggerAction>,
}

pub enum TriggerAction {
    Update(UpdateStatement),
    Insert(InsertStatement),
    Delete(DeleteStatement),
    Select(SelectStatement),
}

pub enum TriggerType {
    On,
    Before,
    After,
    InsteadOf,
}

pub enum TriggerOnAction {
    Delete,
    Insert,
    Update(Option<Vec<String>>),
}

pub struct CreateViewStatement {
    temporary: bool,
    if_not_exists: bool,
    name: String,
    columns: Vec<String>,
    select_statement: SelectStatement,
}

pub struct CreateVirtualTableStatement {
    if_not_exists: bool,
    using: String,
    params: ParamsNary,
}

pub struct DeleteStatement {
    recursive: bool,
    table_expressions: Vec<CommonTableExpression>,
    name: FullyQualifiedTableName,
    where_clause: WhereClause,
    returning: ReturningClause,
}

pub struct DetachStatement {
    name: String,
}

pub struct DropStatement {
    if_exists: bool,
    target_type: DropTargetType,
    name: String,
}

pub enum DropTargetType {
    Index,
    Table,
    Trigger,
    View,
}

pub struct InsertStatement {
    recursive: bool,
    table_expressions: Option<Vec<CommonTableExpression>>,
    table_name: String,
    table_alias: Option<String>,
    column_names: Option<Vec<String>>,
    source: InsertSource,
    upsert_clause: Option<UpsertClause>,
    returning: ReturningClause,
}

pub enum InsertAction {
    Replace,
    Insert(Option<ConflictActionClause>),
}

pub enum InsertSource {
    Values(Option<Vec<Expression>>),
    Select(SelectStatement),
    Default,
}

pub struct PragmaStatement {
    name: String,
    value: PragmaValue,
}

pub enum PragmaValue {
    Name(String),
    Literal(Literal),
}

pub struct ReindexStatement {
    name: String,
}

pub struct RollbackStatement {
    savepoint_name: Option<String>,
}

pub struct SavepointStatement {
    name: String,
}

pub struct ReleaseStatement {
    name: String,
}

pub struct SelectStatement {
    recursive: bool,
    table_expressions: Option<Vec<CommonTableExpression>>,
    select_source: SelectSource,
    order_clause: Option<OrderClause>,
    limit_clause: Option<LimitClause>,
}

pub enum SelectSource {
    SelectClause(SelectClause),
    ValueList(Vec<ParamsNary>),
}

pub struct SelectClause {
    columns: Vec<String>,
    from_clause: FromClause,
    where_clause: WhereClause,
    group_by: Option<Vec<Expression>>,
    having: Option<Vec<Expression>>,
    windows: Option<Vec<String>>,
}

pub enum SelectType {
    Normal,
    Distinct,
    All,
}

pub struct UpdateStatement {
    recursive: bool,
    table_expressions: Option<Vec<CommonTableExpression>>,
    conflict_action: ConflictActionClause,
    name: String,
    from_clause: Option<FromClause>,
    where_clause: Option<WhereClause>,
    returning_clause: Option<ReturningClause>,
}

pub struct VacuumStatement {}

pub struct OrderClause {}

pub struct LimitClause {}

pub struct FromClause {}

pub struct IndexedColumn {}

pub struct TableConstraint {}

pub struct UpsertClause {}
pub struct WhenClause {}

pub enum Literal {}
pub struct ConflictActionClause {}
pub struct TableOptions {}
pub struct FullyQualifiedTableName {}
pub struct WhereClause {}
pub struct ReturningClause {}
pub struct ParamsNary {}

pub struct ColumnDef {}

pub struct Expression {}

pub struct CommonTableExpression {}
