#[cfg(test)]
mod test {
    use crate::db::{objects::database::Database, producer::postgres::PostgresStatementProducer};

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
            "TABLE test (id INT PRIMARY KEY, name TEXT NOT NULL) PARTITION;"
        );
    }

    #[test]
    fn create_db() {
        let db = Database::<PostgresStatementProducer>::new("test");
        assert_eq!(format!("{db}"), "DATABASE test;");
    }
}
