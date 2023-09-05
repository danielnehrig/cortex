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
            let t = Self::serialize_prop_type(&x.t_type);
            if let Some(a) = &x.annotation {
                let a = Self::serialize_prop_annotation(a);
                props.push(format!("{} {} {}", x.name, t, a));
            } else {
                props.push(format!("{} {}", x.name, t));
            }
        }
        for x in &table.annotations {
            let t = Self::serialize_table_annotation(x);
            annotations.push(t);
        }
        format!(
            "CREATE TABLE {} ({}){};",
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

    fn serialize_prop_annotation(t: &crate::db::objects::table::PropAnnotation) -> String {
        match t {
            crate::db::objects::table::PropAnnotation::PrimaryKey => "PRIMARY KEY".to_string(),
            crate::db::objects::table::PropAnnotation::Unique => todo!(),
            crate::db::objects::table::PropAnnotation::NotNull => todo!(),
            crate::db::objects::table::PropAnnotation::Default => todo!(),
            crate::db::objects::table::PropAnnotation::Check => todo!(),
            crate::db::objects::table::PropAnnotation::Foreign => todo!(),
        }
    }

    fn serialize_table_annotation(t: &crate::db::objects::table::TableAnnotation) -> String {
        match t {
            crate::db::objects::table::TableAnnotation::Partition => "PARTITION".to_string(),
            crate::db::objects::table::TableAnnotation::View => todo!(),
        }
    }
}
