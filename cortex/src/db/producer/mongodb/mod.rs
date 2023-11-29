use std::fmt::{Display, Formatter};

use crate::{
    connection::{mongodb::Mongo, ExecuteError, ExecuteType},
    objects::step::Step,
};

mod database;
mod statement;
mod step;
mod table;

pub struct MongodbStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
    connection: Mongo,
}

impl<'a> MongodbStatementProducer<'a> {
    pub fn new(connection: Mongo) -> Self {
        Self {
            data: Vec::new(),
            connection,
        }
    }

    pub fn add_step(mut self, step: Step<'a, Self>) -> Self {
        self.data.push(step);
        self
    }

    pub fn clean(mut self) -> Self {
        self.data.clear();
        self
    }

    pub fn execute(mut self) -> Result<Self, ExecuteError> {
        for step in self.data {
            for statement in step.statements {
                self.connection.execute(ExecuteType::Driver(statement))?;
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection,
        })
    }
}

impl Display for MongodbStatementProducer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for step in &self.data {
            writeln!(f, "{}", step)?;
        }
        Ok(())
    }
}
