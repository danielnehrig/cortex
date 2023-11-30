use crate::{
    connection::{mongodb::Mongo, ExecuteError, ExecuteType},
    objects::step::Step,
};

pub struct CortexMongo<'a> {
    data: Vec<Step<'a>>,
    connection: Mongo,
}

impl<'a> CortexMongo<'a> {
    pub fn new(connection: Mongo) -> Self {
        Self {
            data: Vec::new(),
            connection,
        }
    }

    pub fn add_step(mut self, step: Step<'a>) -> Self {
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
