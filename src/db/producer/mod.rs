use crate::db::objects::table::{PropAnnotation, PropType, Table, TableAnnotation};

#[cfg(feature = "postgres")]
pub mod postgres;

pub trait StatementProducer<T: DatabaseSpeicifics + Clone> {
    fn create_table(&self, table: &Table<T>) -> String;
}

pub trait DatabaseSpeicifics
where
    Self: Sized,
{
    fn serialize_prop_type(t: &PropType) -> String;
    fn serialize_prop_annotation(t: &PropAnnotation) -> String;
    fn serialize_table_annotation(t: &TableAnnotation) -> String;
}
