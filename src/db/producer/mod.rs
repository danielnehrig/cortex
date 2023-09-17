use crate::db::objects::table::Table;

#[cfg(feature = "mongodb")]
pub mod mongodb;
#[cfg(feature = "postgres")]
pub mod postgres;

pub trait StatementProducer<T> {
    fn create_table(&self, table: &Table<T>) -> String;
}
