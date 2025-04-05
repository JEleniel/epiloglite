use super::{
    expression::Expression,
    functions::operators::CollationName,
    literals::{Level2Name, QualifiedTableName, TypeName},
    statements::select::SelectStatment,
};

pub enum CascadeAction {
    SetNull,
    SetDefault,
    Cascade,
    Restrict,
    NoAction,
}

pub struct ColumnConstraint {
    pub name: Option<String>,
}

pub enum ColumnConstraintType {
    PrimaryKey(PrimaryKeyConstraint),
    NotNull(ConflictAction),
    Unique(ConflictAction),
    Check(Expression),
    Default(Expression),
    Collate(CollationName),
    ForeignKey(ForeignKeyClause),
    Generated(GeneratedClause),
}

pub struct ColumnDef {
    pub column_name: String,
    pub type_name: Option<TypeName>,
    pub column_constraing: Option<Vec<ColumnConstraint>>,
}

pub struct CommonTableExpression {
    pub table_name: String,
    pub column_names: Option<Vec<String>>,
    pub as_materialized: Option<bool>,
    pub select_statement: SelectStatment,
}

pub enum ConflictAction {
    Abort,
    Fail,
    Ignore,
    Replace,
    Rollback,
}

pub enum DeferralType {
    Not,
    NotInitiallyDeferred,
    NotImmediatelyDeferred,
    Deferred,
    InitiallyDeferred,
    ImmediatelyDeferred,
}

pub struct FilteringClause {
    pub where_expr: Expression,
}

pub struct ForeignKeyClause {
    pub foreign_table: String,
    pub foreign_columns: Option<Vec<String>>,
    pub on_delete_action: Option<CascadeAction>,
    pub on_update_action: Option<CascadeAction>,
    pub match_name: Option<String>,
    pub deferral_type: Option<DeferralType>,
}

pub struct GeneratedClause {
    pub generated: bool,
    pub generation_expr: Expression,
    pub generation_type: Option<GenerationType>,
}

pub enum GenerationType {
    Stored,
    Virtual,
}

pub struct IndexedBy {
    pub table_name: Level2Name,
    pub alias: Option<String>,
    pub indexed_by_type: IndexedByType,
}

pub enum IndexedByType {
    NonIndexed,
    IndexedBy(String),
}

pub struct IndexedColumn {
    pub indexed_column_type: IndexedColumnType,
    pub collation_name: Option<CollationName>,
    pub sort_direction: Option<SortDirection>,
}

pub enum IndexedColumnType {
    Name(String),
    Expression(Expression),
}

pub struct OrderingTerm {
    pub expr: Expression,
    pub collation_name: Option<CollationName>,
    pub sort_direction: Option<SortDirection>,
    pub nulls_order: Option<NullsOrder>,
}

pub struct PrimaryKeyConstraint {
    pub sort_direction: Option<SortDirection>,
    pub conflict_clause: ConflictAction,
    pub autoincrement: bool,
}

pub struct ReturningClause {
    pub returning: ReturningType,
}

pub struct ReturningItem {
    pub expr: Expression,
    pub as_name: Option<String>,
}

pub enum ReturningType {
    All,
    Items(Vec<ReturningItem>),
}

pub struct TableConstraint {
    pub name: Option<String>,
    pub table_constraint_type: TableConstraintType,
}

pub enum TableConstraintType {
    PrimaryKey(Vec<IndexedColumn>, ConflictAction),
    Unique(Vec<IndexedColumn>, ConflictAction),
    Check(Expression),
    ForeignKey(Vec<String>, ForeignKeyClause),
}

pub struct TableDefClause {
    pub column_defs: Vec<ColumnDef>,
    pub table_constraints: Option<Vec<TableConstraint>>,
    pub table_options: Option<TableOptions>,
}

pub struct TableOptions {
    pub without_rowid: bool,
    pub strict: bool,
}

pub enum UpsertAction {
    Nothing,
    Set(Vec<SetClause>),
}

pub struct UpsertClause {
    pub indexed_column_list: Option<Vec<IndexedColumn>>,
    pub target_where_expr: Option<Expression>,
    pub upsert_action: UpsertAction,
    pub source_where_expr: Option<Expression>,
}

pub struct SetClause {
    pub column_name: Vec<String>,
    pub value: Expression,
}

pub enum FromClause {
    TableOrSubquery(Vec<TableOrSubquery>),
    Join(JoinClause),
}

pub enum TableOrSubquery {
    Table(QualifiedTableName),
    TableFunction(String, Vec<Expression>, String),
    Select(Box<SelectStatment>),
    TableOrSubquery(Box<TableOrSubquery>),
    Join(Box<JoinClause>),
}

pub struct JoinClause {
    pub table_or_subquery: TableOrSubquery,
    pub join_operation: Vec<JoinOperator>,
}

pub enum JoinOperator {
    Comma(TableOrSubquery, JoinConstraint),
    Natural(TableOrSubquery, JoinConstraint),
    Left(TableOrSubquery, JoinConstraint),
    NaturalLeft(TableOrSubquery, JoinConstraint),
    LeftOuter(TableOrSubquery, JoinConstraint),
    NaturalLeftOuter(TableOrSubquery, JoinConstraint),
    Right(TableOrSubquery, JoinConstraint),
    NaturalRight(TableOrSubquery, JoinConstraint),
    RightOuter(TableOrSubquery, JoinConstraint),
    NaturalRightOuter(TableOrSubquery, JoinConstraint),
    Full(TableOrSubquery, JoinConstraint),
    NaturalFull(TableOrSubquery, JoinConstraint),
    FullOuter(TableOrSubquery, JoinConstraint),
    NaturalFullOuter(TableOrSubquery, JoinConstraint),
    Inner(TableOrSubquery, JoinConstraint),
    NaturalInner(TableOrSubquery, JoinConstraint),
    Cross(TableOrSubquery, JoinConstraint),
}

pub enum JoinConstraint {
    On(Expression),
    Using(Vec<String>),
}

pub struct CastClause {
    pub expr: Box<Expression>,
    pub type_name: TypeName,
}

pub enum SortDirection {
    Ascending,
    Descending,
}

pub enum NullsOrder {
    First,
    Last,
}
