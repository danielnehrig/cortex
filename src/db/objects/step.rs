use crate::db::{objects::statement::Statement, producer::DatabaseSpeicifics};

#[derive(Debug, Clone)]
pub struct Step<T: DatabaseSpeicifics + Clone> {
    pub name: &'static str,
    pub s_type: StepType,
    pub statements: Vec<Statement<T>>,
}

#[derive(Debug, Clone)]
pub enum StepType {
    Update,
}

impl<T: DatabaseSpeicifics + Clone> Step<T> {
    pub fn new(name: &'static str, s_type: StepType) -> Self {
        Self {
            name,
            s_type,
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement<T>) -> Self {
        self.statements.push(statement);
        self.to_owned()
    }
}
