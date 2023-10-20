use std::fmt::{Display, Formatter};

use postgres::Client;

use crate::objects::step::Step;

mod database;
mod statement;
mod step;
mod table;
mod tests;

pub struct PostgresStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
}

impl<'a> PostgresStatementProducer<'a> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_step(mut self, step: Step<'a, Self>) -> Self {
        self.data.push(step);
        self
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
