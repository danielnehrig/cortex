use crate::db::{objects::table::Table, producer::DatabaseSpeicifics};

#[derive(Debug, Clone)]
pub enum Statement<'a, T: DatabaseSpeicifics + Clone> {
    Create(CreateObject<'a, T>),
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
pub enum CreateObject<'a, T: DatabaseSpeicifics + Clone> {
    Table(Table<'a, T>),
    Sequence,
    Database,
    Role,
    User,
}
