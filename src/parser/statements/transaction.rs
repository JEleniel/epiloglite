pub enum TransactionStatement {
    Begin(TransactionType),
    Commit,
    Rollback(Option<String>),
}

pub enum TransactionType {
    Normal,
    Deferred,
    Immediate,
    Exclusive,
}
