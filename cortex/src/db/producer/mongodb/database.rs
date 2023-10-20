use std::fmt::Display;

use crate::{
    objects::{
        database::Database,
        statement::{CreateableObject, DropableObject},
    },
    producer::MongodbStatementProducer,
};

impl CreateableObject for Database<MongodbStatementProducer<'_>> {
    fn create(&self) -> String {
        format!("use {};", self.name)
    }
}

impl DropableObject for Database<MongodbStatementProducer<'_>> {
    fn drop(&self) -> String {
        format!("{}.dropDatabase();", self.name)
    }
}

impl Display for Database<MongodbStatementProducer<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create())
    }
}
