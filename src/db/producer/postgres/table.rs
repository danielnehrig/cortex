use std::fmt::Display;

use crate::{
    db::producer::DatabaseSpeicifics,
    objects::{
        statement::{CreateableObject, DropableObject, InsertableObject},
        table::{PropAnnotation, PropType, Table, TableAnnotation},
    },
    producer::PostgresStatementProducer,
};

impl CreateableObject for Table<PostgresStatementProducer<'_>> {
    fn create(&self) -> String {
        let props = &self
            .props
            .iter()
            .map(|x| {
                let t = PostgresStatementProducer::serialize_prop_type(&x.t_type);
                match &x.annotation.clone() {
                    Some(p) => {
                        let a = PostgresStatementProducer::serialize_prop_annotation(p);
                        format!("{} {} {}", x.name, t, a)
                    }
                    None => {
                        format!("{} {}", x.name, t)
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        let annotations = &self
            .annotations
            .iter()
            .map(PostgresStatementProducer::serialize_table_annotation)
            .collect::<Vec<String>>()
            .join(" ");

        match (props.is_empty(), annotations.is_empty()) {
            (true, true) => format!("TABLE {};", self.name),
            (true, false) => format!("TABLE {} {};", self.name, annotations),
            (false, true) => format!("TABLE {} ({});", self.name, props),
            (false, false) => format!("TABLE {} ({}) {};", self.name, props, annotations),
        }
    }
}

impl InsertableObject for Table<PostgresStatementProducer<'_>> {
    fn insert(&self) -> String {
        // let props = &self
        // .props
        // .iter()
        // .map(|x| x.name.to_string())
        // .collect::<Vec<String>>()
        // .join(", ");
        todo!();
        // format!("INSERT INTO {} ({}) VALUES ({});", self.name, props, values)
    }
}

impl DropableObject for Table<PostgresStatementProducer<'_>> {
    fn drop(&self) -> String {
        format!("TABLE {};", self.name)
    }
}

impl Display for Table<PostgresStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create())
    }
}

impl DatabaseSpeicifics for PostgresStatementProducer<'_> {
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
            PropAnnotation::Constraint(_) => todo!(),
        }
    }

    fn serialize_table_annotation(t: &TableAnnotation) -> String {
        match t {
            TableAnnotation::Partition => "PARTITION".to_string(),
            TableAnnotation::View => {
                todo!()
            }
        }
    }
}
