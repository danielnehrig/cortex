use std::fmt::Display;

use crate::{
    objects::{
        database::Database,
        statement::{CreateableObject, DropableObject},
    },
    producer::PostgresStatementProducer,
};

impl CreateableObject for Database<'_, PostgresStatementProducer<'_>> {
    fn create(&self) -> String {
        format!("DATABASE {};", self.name)
    }
}

impl DropableObject for Database<'_, PostgresStatementProducer<'_>> {
    fn drop(&self) -> String {
        format!("DATABASE {};", self.name)
    }
}

impl Display for Database<'_, PostgresStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create())
    }
}
