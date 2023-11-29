use std::rc::Rc;

use crate::db::objects::statement::Statement;

#[derive(Clone)]
pub struct Step<'a, T> {
    pub name: Rc<str>,
    pub s_type: StepType,
    pub statements: Vec<Statement<'a, T>>,
    pub version: semver::Version,
}

#[derive(Default, Debug, Clone)]
pub enum StepType {
    /// The step is a setup step, which is run before the main update step.
    InitSetup,
    /// The step is a update step which is run after the setup step.
    #[default]
    Update,
}

impl<'a, T> Step<'a, T> {
    pub fn new(name: &'a str, s_type: StepType, version: semver::Version) -> Self {
        Self {
            name: Rc::from(name),
            s_type,
            statements: Vec::new(),
            version,
        }
    }

    pub fn add_statement(mut self, statement: Statement<'a, T>) -> Self {
        self.statements.push(statement);
        self
    }
}
