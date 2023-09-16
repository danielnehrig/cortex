use std::rc::Rc;

use crate::db::{objects::statement::Statement, producer::DatabaseSpeicifics};

#[derive(Clone)]
pub struct Step<'a, T: DatabaseSpeicifics> {
    pub name: Rc<str>,
    pub s_type: StepType,
    pub statements: Vec<Statement<'a, T>>,
}

#[derive(Debug, Clone)]
pub enum StepType {
    Update,
}

impl<'a, T: DatabaseSpeicifics> Step<'a, T> {
    pub fn new(name: &'a str, s_type: StepType) -> Self {
        Self {
            name: Rc::from(name),
            s_type,
            statements: Vec::new(),
        }
    }

    pub fn add_statement(mut self, statement: Statement<'a, T>) -> Self {
        self.statements.push(statement);
        self
    }
}
