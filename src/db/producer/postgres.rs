use std::fmt::Display;

use crate::db::{
    objects::{
        statement::{CreateableObject, DropableObject, Statement},
        step::Step,
        table::{PropAnnotation, PropType, Table, TableAnnotation},
    },
    producer::DatabaseSpeicifics,
};

#[derive(Debug, Clone)]
pub struct PostgresStatementProducer;

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
            PropAnnotation::Constraint(_) => todo!(),
        }
    }

    fn serialize_table_annotation(t: &TableAnnotation) -> String {
        match t {
            TableAnnotation::Partition => "PARTITION".to_string(),
            TableAnnotation::View => todo!(),
        }
    }
}

impl Display for Statement<'_, crate::db::producer::postgres::PostgresStatementProducer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Create(x) => write!(
                f,
                "CREATE {}",
                match x {
                    CreateableObject::Table(x) => format!("{}", x),
                }
            ),
            Statement::Drop(x) => write!(
                f,
                "DROP {}",
                match x {
                    DropableObject::Table(x) => format!("{}", x),
                }
            ),
        }
    }
}

impl Display for Step<'_, PostgresStatementProducer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = &self
            .statements
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}\n{}", self.name, statements)
    }
}

impl<'a> Display for Table<'a, PostgresStatementProducer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let props = &self
            .props
            .iter()
            .map(|x| {
                let t = PostgresStatementProducer::serialize_prop_type(&x.t_type);
                let a = PostgresStatementProducer::serialize_prop_annotation(
                    &x.annotation.clone().unwrap_or_default(),
                );
                format!("{} {} {}", x.name, t, a)
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
            (true, true) => write!(f, "TABLE {};", self.name),
            (true, false) => write!(f, "TABLE {} {};", self.name, annotations),
            (false, true) => write!(f, "TABLE {} ({});", self.name, props),
            (false, false) => write!(f, "TABLE {} ({}) {};", self.name, props, annotations),
        }
    }
}
