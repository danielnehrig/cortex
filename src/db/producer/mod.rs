use crate::db::objects::table::{PropType, Table};

//#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub trait StatementProducer<T: DatabaseSpeicifics + Clone> {
    fn create_table(&self, table: &Table<T>) -> String;
}

pub trait DatabaseSpeicifics
where
    Self: Sized,
{
    fn serialize_type(t: &PropType) -> String;
}
