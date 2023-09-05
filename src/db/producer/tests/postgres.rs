#[cfg(test)]
mod test {
    #[test]
    fn create_table_1() {
        use crate::db::{
            objects::table::{PropAnnotation, PropType, Table, TableAnnotation, TableProp},
            producer::{postgres::PostgresStatementProducer, StatementProducer},
        };
        let mut table = Table::new("test");
        let table = table
            .add_prop(TableProp::new(
                "id",
                PropType::Int,
                Some(PropAnnotation::PrimaryKey),
            ))
            .add_prop(TableProp::new(
                "name",
                PropType::Text,
                Some(PropAnnotation::NotNull),
            ))
            .add_annotation(TableAnnotation::Partition);
        let producer = PostgresStatementProducer;
        let statement = producer.create_table(&table);
        assert_eq!(
            statement,
            "CREATE TABLE test (id INT PRIMARY KEY, name TEXT NOT NULL) PARTITION;"
        );
    }
}
