use std::fmt::Display;

use crate::{
    objects::{
        statement::{CreateableObject, DropableObject, InsertableObject},
        table::{PropAnnotation, PropType, Table, TableAnnotation, TableProp},
    },
    producer::MongodbStatementProducer,
};

impl CreateableObject for Table<MongodbStatementProducer<'_>> {
    fn create(&self) -> String {
        let props = &self
            .props
            .iter()
            .map(|x| x.compose())
            .collect::<Vec<String>>()
            .join(", ");
        format!("db.createCollection(\"{}\", {{ {} }});", self.name, props)
    }
}

impl InsertableObject for Table<MongodbStatementProducer<'_>> {
    fn insert(&self) -> String {
        let props = &self
            .props
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("db.{}.insert({{ {} }});", self.name, props)
    }
}

impl DropableObject for Table<MongodbStatementProducer<'_>> {
    fn drop(&self) -> String {
        format!("db.{}.drop();", self.name)
    }
}

impl Display for Table<MongodbStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create())
    }
}

impl Table<MongodbStatementProducer<'_>> {
    pub fn serialize_annotation(annotations: &TableAnnotation) -> String {
        match annotations {
            TableAnnotation::Partition => "PARTITION".to_string(),
            TableAnnotation::View => "VIEW".to_string(),
        }
    }
}

impl TableProp<MongodbStatementProducer<'_>> {
    pub fn compose(&self) -> String {
        let mut prop = format!("{} {}", self.name, self.serialize_prop_type());
        if let Some(annotation) = &self.annotation {
            prop = format!(
                "{} {}",
                prop,
                TableProp::serialize_prop_annotation(annotation.clone())
            );
        }
        prop
    }

    fn serialize_prop_annotation(prop_annotation: PropAnnotation) -> String {
        match prop_annotation {
            PropAnnotation::PrimaryKey => "PRIMARY KEY".to_string(),
            PropAnnotation::Unique => "UNIQUE".to_string(),
            PropAnnotation::NotNull => "NOT NULL".to_string(),
            PropAnnotation::Default => todo!(),
            PropAnnotation::Check => todo!(),
            PropAnnotation::Foreign => todo!(),
            PropAnnotation::Constraint(_) => todo!(),
            PropAnnotation::Empty => todo!(),
        }
    }

    fn serialize_prop_type(&self) -> String {
        match self.t_type {
            PropType::Int => "int".to_string(),
            PropType::Bigint => "int".to_string(),
            PropType::Smallint => "int".to_string(),
            PropType::Text => "string".to_string(),
            PropType::Double => "float".to_string(),
            PropType::Bool => "boolean".to_string(),
            PropType::Date => "date".to_string(),
            PropType::Timestamp => todo!(),
        }
    }
}
