use serde::{Deserialize, Serialize};

/// ON CONFLICT actions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictClause {
    None,
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace,
}
