use crate::db::{objects::table::Table, producer::DatabaseSpeicifics};

#[derive(Debug, Clone)]
pub enum Statement<T: DatabaseSpeicifics + Clone> {
    Create(CreateObject<T>),
    Drop,
    Alter,
    Insert,
    Select,
    Update,
    Delete,
    Truncate,
    Comment,
    Merge,
    Call,
    Explain,
    Show,
    Use,
    Set,
    Commit,
    Rollback,
    Savepoint,
    Release,
    Lock,
    Unlock,
    Rename,
    Grant,
    Revoke,
    Analyze,
    Refresh,
    Describe,
    Discard,
    Other,
}

#[derive(Debug, Clone)]
pub enum CreateObject<T: DatabaseSpeicifics + Clone> {
    Table(Table<T>),
    Sequence,
    Database,
    Role,
    User,
}
