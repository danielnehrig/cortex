use std::fmt::{Display, Formatter};

use crate::objects::step::Step;

pub mod database;
pub mod statement;
pub mod step;
pub mod table;
mod tests;

#[derive(Clone, Default)]
pub struct PostgresStatementProducer<'a>(Vec<Step<'a, Self>>);

impl<'a> PostgresStatementProducer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_step(mut self, step: Step<'a, Self>) -> Self {
        self.0.push(step);
        self
    }
}

impl Display for PostgresStatementProducer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for step in &self.0 {
            writeln!(f, "{}", step)?;
        }
        Ok(())
    }
}
