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

    pub fn add_step(&mut self, step: Step<'a, Self>) -> &mut Self {
        self.0.push(step);
        self
    }
}
