pub mod alter_table;
pub mod analyze;
pub mod attach_detach;
pub mod create;
pub mod delete;
pub mod drop;
pub mod explain;
pub mod insert;
pub mod pragma;
pub mod reindex;
pub mod savepoint;
pub mod select;
pub mod transaction;
pub mod update;

use alter_table::AlterTableStatement;
use analyze::AnalyzeStatement;
use attach_detach::{AttachStatement, DetachStatement};
use create::CreateStatement;
use delete::DeleteStatement;
use drop::DropStatement;
use explain::ExplainStatement;
use insert::InsertStatement;
use pragma::PragmaStatement;
use reindex::ReindexStatement;
use savepoint::{ReleaseStatement, SavepointStatement};
use select::SelectStatment;
use transaction::TransactionStatement;
use update::UpdateStatement;

pub struct SqlProgram {
    pub statements: Vec<SqlStatement>,
}

pub enum SqlStatement {
    AlterTable(AlterTableStatement),
    Analyze(AnalyzeStatement),
    Attach(AttachStatement),
    Create(CreateStatement),
    Delete(DeleteStatement),
    Detach(DetachStatement),
    Drop(DropStatement),
    Explain(ExplainStatement),
    Insert(InsertStatement),
    Pragma(PragmaStatement),
    Reindex(ReindexStatement),
    Select(SelectStatment),
    Savepoint(SavepointStatement),
    Release(ReleaseStatement),
    Transaction(TransactionStatement),
    Update(UpdateStatement),
}
