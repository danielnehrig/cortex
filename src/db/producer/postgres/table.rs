use std::fmt::Display;

use crate::{
    objects::{
        statement::{CreateableObject, DropableObject, InsertableObject},
        table::{PropAnnotation, PropType, Table, TableAnnotation, TableProp},
    },
    producer::PostgresStatementProducer,
};

impl CreateableObject for Table<PostgresStatementProducer<'_>> {
    fn create(&self) -> String {
        let props = &self
            .props
            .iter()
            .map(TableProp::compose)
            .collect::<Vec<String>>()
            .join(", ");
        let annotations = &self
            .annotations
            .iter()
            .map(Table::serialize_annotation)
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

impl Table<PostgresStatementProducer<'_>> {
    pub fn serialize_annotation(annotations: &TableAnnotation) -> String {
        match annotations {
            TableAnnotation::Partition => "PARTITION".to_string(),
            TableAnnotation::View => "VIEW".to_string(),
        }
    }
}

impl TableProp<PostgresStatementProducer<'_>> {
    pub fn compose(&self) -> String {
        let t = &self.serialize_prop_type();
        match &self.annotation.clone() {
            Some(p) => {
                let a = &TableProp::serialize_prop_annotation(p.clone());
                format!("{} {} {}", self.name, t, a)
            }
            None => {
                format!("{} {}", self.name, t)
            }
        }
    }

    fn serialize_prop_annotation(prop_annotation: PropAnnotation) -> String {
        match prop_annotation {
            PropAnnotation::PrimaryKey => "PRIMARY KEY".to_string(),
            PropAnnotation::Unique => "UNIQUE".to_string(),
            PropAnnotation::NotNull => "NOT NULL".to_string(),
            PropAnnotation::Default => "DEFAULT".to_string(),
            PropAnnotation::Check => "CHECK".to_string(),
            PropAnnotation::Foreign => "FOREIGN".to_string(),
            PropAnnotation::Constraint(_) => "CONSTRAINT".to_string(),
            PropAnnotation::Empty => "".to_string(),
        }
    }

    fn serialize_prop_type(&self) -> String {
        match self.t_type {
            PropType::Int => "INT".to_string(),
            PropType::Double => "DOUBLE".to_string(),
            PropType::Text => "TEXT".to_string(),
            PropType::Bool => "BOOL".to_string(),
            PropType::Date => "DATE".to_string(),
            PropType::Timestamp => "TIMESTAMP".to_string(),
            PropType::Bigint => "BIGINT".to_string(),
            PropType::Smallint => "SMALLINT".to_string(),
        }
    }
}
