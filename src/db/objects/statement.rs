use crate::db::{objects::table::Table, producer::DatabaseSpeicifics};

#[derive(Debug, Clone)]
pub enum Statement<'a, T: DatabaseSpeicifics> {
    Create(CreateableObject<'a, T>),
    Drop(DropableObject<'a, T>),
}

#[derive(Debug, Clone)]
pub enum CreateableObject<'a, T: DatabaseSpeicifics> {
    Table(Table<'a, T>),
}

#[derive(Debug, Clone)]
pub enum DropableObject<'a, T: DatabaseSpeicifics> {
    Table(Table<'a, T>),
}
