use std::fmt::{Display, Formatter};

use crate::objects::step::Step;

mod database;
mod statement;
mod step;
mod table;

pub struct SQLiteStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
}

impl<'a> SQLiteStatementProducer<'a> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_step(mut self, step: Step<'a, Self>) -> Self {
        self.data.push(step);
        self
    }
}

impl<'a> Default for SQLiteStatementProducer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SQLiteStatementProducer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for step in &self.data {
            writeln!(f, "{}", step)?;
        }
        Ok(())
    }
}
