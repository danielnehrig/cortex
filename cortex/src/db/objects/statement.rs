use crate::{
    objects::{database::Database, table::Table, user::User, views::View},
    prelude::{Role, Sequence},
};

#[derive(Clone)]
/// A database action is an action that is run on the database.
pub enum DbAction {
    Create,
    Drop,
    Alter,
    Insert,
    Grant,
    Revoke,
}

#[derive(Clone)]
/// A statement is a single action that is run on the database.
pub enum Statement {
    Table(Table),
    Database(Database),
    View(View),
    User(User),
    Role(Role),
    Sequence(Sequence),
}
