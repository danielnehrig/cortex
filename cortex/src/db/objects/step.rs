use std::rc::Rc;

use crate::db::objects::statement::Statement;

#[derive(Clone)]
/// Steps are the main unit of work in the migration system.
/// They are run in order, and can be either setup steps or update steps.
/// Setup steps are run before the main update step, and are used to create
/// the database and tables. Update steps are run after the setup step, and
/// are used to update the database and tables.
/// Every step can have a description, which is used to describe the step
/// The version is used to determine the order of the steps
/// and also if the step has already been run.
pub struct Step<'a> {
    /// The name of the step
    pub name: Rc<str>,
    /// The type of the step
    pub s_type: StepType,
    /// The statements that are run in the step
    pub statements: Vec<Statement<'a>>,
    /// The version of the step
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

impl<'a> Step<'a> {
    /// Create a new step with the given name, type and version.
    pub fn new(name: &str, s_type: StepType, version: semver::Version) -> Self {
        Self {
            name: Rc::from(name),
            s_type,
            statements: Vec::new(),
            version,
        }
    }

    /// Add a statement to the step.
    pub fn add_statement(mut self, statement: Statement<'a>) -> Self {
        self.statements.push(statement);
        self
    }
}
