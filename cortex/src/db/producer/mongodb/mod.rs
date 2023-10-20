// use std::fmt::{Display, Formatter};

use std::fmt::{Display, Formatter};

use mongodb::{bson::doc, Client};

use crate::objects::step::Step;

pub mod database;
pub mod statement;
pub mod step;
pub mod table;

pub struct MongodbStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
}

impl<'a> MongodbStatementProducer<'a> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_step(mut self, step: Step<'a, Self>) -> Self {
        self.data.push(step);
        self
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
