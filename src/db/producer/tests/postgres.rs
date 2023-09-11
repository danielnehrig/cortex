#[cfg(test)]
mod test {
    use crate::db::producer::postgres::PostgresStatementProducer;

    #[test]
    fn create_table_1() {
        use crate::db::objects::table::{
            PropAnnotation, PropType, Table, TableAnnotation, TableProp,
        };
        let table = Table::<PostgresStatementProducer>::new("test")
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
        assert_eq!(
            format!("{table}"),
            "CREATE TABLE test (id INT PRIMARY KEY, name TEXT NOT NULL) PARTITION;"
        );
    }
}
