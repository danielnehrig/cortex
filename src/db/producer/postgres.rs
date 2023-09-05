use crate::db::{
    objects::table::{PropAnnotation, PropType, Table, TableAnnotation},
    producer::{DatabaseSpeicifics, StatementProducer},
};

#[derive(Debug, Clone)]
pub struct PostgresStatementProducer;

impl StatementProducer<PostgresStatementProducer> for PostgresStatementProducer {
    /// # Examples
    ///
    /// ```
    /// use schemacreator::db::{
    /// objects::{
    /// statement::{CreateObject, Statement},
    /// step::{Step, StepType},
    /// table::{PropType, Table, TableProp, TableAnnotation, PropAnnotation},
    /// },
    /// producer::{postgres::PostgresStatementProducer, StatementProducer},
    /// };
    /// let mut table = Table::new(
    ///   "test",
    /// );
    /// let mut table = table
    ///  .add_prop(TableProp::new("id", PropType::Int, Some(PropAnnotation::PrimaryKey)))
    ///  .add_prop(TableProp::new("name", PropType::Text, Some(PropAnnotation::NotNull)))
    ///  .add_annotation(TableAnnotation::Partition);
    ///    let producer = PostgresStatementProducer;
    ///    let statement = producer.create_table(&table);
    ///    assert_eq!(statement, "CREATE TABLE test (id INT PRIMARY KEY, name TEXT NOT NULL) PARTITION;");
    /// ```
    fn create_table(&self, table: &Table<PostgresStatementProducer>) -> String {
        let mut props = vec![];
        let mut annotations = vec![];
        for x in &table.props {
            let t = Self::serialize_prop_type(&x.t_type);
            let a = Self::serialize_prop_annotation(
                &x.annotation.clone().map(|x| x).unwrap_or_default(),
            );
            props.push(format!("{} {} {}", x.name, t, a));
        }
        for x in &table.annotations {
            let t = Self::serialize_table_annotation(x);
            annotations.push(t);
        }
        format!(
            "CREATE TABLE {} ({}) {};",
            table.name,
            props.join(", "),
            annotations.join(", ")
        )
    }
}

impl DatabaseSpeicifics for PostgresStatementProducer {
    fn serialize_prop_type(t: &PropType) -> String {
        match t {
            PropType::Int => "INT",
            PropType::Double => "DOUBLE",
            PropType::Text => "TEXT",
            PropType::Bool => "BOOL",
            PropType::Date => "DATE",
            PropType::Timestamp => "TIMESTAMP",
            PropType::Bigint => "BIGINT",
            PropType::Smallint => "SMALLINT",
        }
        .to_string()
    }

    fn serialize_prop_annotation(t: &PropAnnotation) -> String {
        match t {
            PropAnnotation::PrimaryKey => "PRIMARY KEY".to_string(),
            PropAnnotation::Unique => todo!(),
            PropAnnotation::NotNull => "NOT NULL".to_string(),
            PropAnnotation::Default => todo!(),
            PropAnnotation::Check => todo!(),
            PropAnnotation::Foreign => todo!(),
            PropAnnotation::Empty => "".to_string(),
        }
    }

    fn serialize_table_annotation(t: &TableAnnotation) -> String {
        match t {
            TableAnnotation::Partition => "PARTITION".to_string(),
            TableAnnotation::View => todo!(),
        }
    }
}
