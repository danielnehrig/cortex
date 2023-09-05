use crate::db::{
    objects::table::PropType,
    producer::{DatabaseSpeicifics, StatementProducer},
};

#[derive(Debug, Clone)]
pub struct PostgresStatementProducer;

impl StatementProducer<PostgresStatementProducer> for PostgresStatementProducer {
    fn create_table(
        &self,
        table: &crate::db::objects::table::Table<PostgresStatementProducer>,
    ) -> String {
        let mut props = vec![];
        let mut annotations = vec![];
        for x in &table.props {
            let t = Self::serialize_type(&x.t_type);
            props.push(format!("{} {}", x.name, t));
        }
        for x in &table.annotations {
            let t = match x {
                crate::db::objects::table::TableAnnotation::Partition => "PARTITION BY",
                crate::db::objects::table::TableAnnotation::View => "VIEW",
            };
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
    fn serialize_type(t: &PropType) -> String {
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
}
