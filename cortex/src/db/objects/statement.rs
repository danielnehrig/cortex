use crate::{
    objects::{
        composite_type::CompositeType, database::Database, table::Table, trigger::Trigger,
        user::User, views::View,
    },
    prelude::{Role, Sequence},
};

#[derive(Clone, Debug, PartialEq)]
/// A database action is an action that is run on the database.
pub enum DbAction {
    Create,
    Drop,
    Alter,
    Insert,
    Grant,
    Revoke,
}

#[derive(Clone, Debug, PartialEq)]
/// A statement is a single action that is run on the database.
pub enum Statement {
    Table(Table),
    Database(Database),
    View(View),
    User(User),
    Role(Role),
    Sequence(Sequence),
    CompositeType(CompositeType),
    Trigger(Trigger),
}

impl Statement {
    pub fn get_as_table(&self) -> Option<&Table> {
        match self {
            Statement::Table(table) => Some(table),
            _ => None,
        }
    }
    pub fn get_as_composite_type(&self) -> Option<&CompositeType> {
        match self {
            Statement::CompositeType(ctype) => Some(ctype),
            _ => None,
        }
    }
}
