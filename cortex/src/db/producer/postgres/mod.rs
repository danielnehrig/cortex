use std::fmt::{Display, Formatter};

use crate::{
    connection::{postgres::Postgres, ExecuteError, ExecuteType},
    objects::step::Step,
};

mod database;
mod statement;
mod step;
mod table;
mod tests;

pub struct PostgresStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
    connection: Postgres,
}

impl<'a> PostgresStatementProducer<'a> {
    pub fn new(connection: Postgres) -> Self {
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
        for step in &self.data {
            for statement in &step.statements {
                self.connection
                    .execute(ExecuteType::Command(statement.to_string()))?;
            }
        }
        Ok(self)
    }
}

impl Display for PostgresStatementProducer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for step in &self.data {
            writeln!(f, "{}", step)?;
        }
        Ok(())
    }
}
