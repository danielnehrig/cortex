use std::rc::Rc;

use crate::{
    db::objects::statement::Statement, objects::statement::DbAction, prelude::ExecutionMode,
};

#[derive(Clone, Debug)]
/// Steps are the main unit of work in the migration system.
/// They are run in order, and can be either setup steps or update steps.
/// Setup steps are run before the main update step, and are used to create
/// the database and tables. Update steps are run after the setup step, and
/// are used to update the database and tables.
/// Every step can have a description, which is used to describe the step
/// The version is used to determine the order of the steps
/// and also if the step has already been run.
pub struct Step {
    /// The name of the step
    pub name: Rc<str>,
    /// The type of the step
    pub s_type: StepType,
    /// The statements that are run in the step
    pub statements: Vec<(Statement, DbAction)>,
    /// The version of the step
    pub version: semver::Version,
    /// Execution Mode
    pub mode: ExecutionMode,
}

#[derive(Default, Debug, Clone)]
pub enum StepType {
    /// The step is a setup step, which is run before the main update step.
    InitSetup,
    /// The step is a update step which is run after the setup step.
    #[default]
    Update,
}

impl Step {
    /// Create a new step with the given name, type and version.
    pub fn new(name: &str, s_type: StepType, version: semver::Version) -> Self {
        Self {
            name: Rc::from(name),
            s_type,
            statements: Vec::new(),
            version,
            mode: ExecutionMode::Optimistic,
        }
    }

    /// Set the execution mode of the step.
    /// # Example
    /// ```
    /// use cortex::objects::step::{Step, StepType};
    /// use cortex::prelude::ExecutionMode;
    /// let step = Step::new("test", StepType::Update, semver::Version::new(1, 0, 0))
    ///   .set_execution_mode(ExecutionMode::Optimistic);
    /// ```
    pub fn set_execution_mode(mut self, mode: ExecutionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Add a statement to the step.
    /// # Example
    /// ```
    /// use cortex::objects::step::{Step, StepType};
    /// use cortex::objects::statement::Statement;
    /// use cortex::objects::table::Table;
    /// use cortex::objects::statement::DbAction;
    ///
    /// let table = Table::new("test");
    /// let step = Step::new("test", StepType::Update, semver::Version::new(1, 0, 0))
    ///    .add_statement(table, DbAction::Create);
    /// ```
    pub fn add_statement(mut self, statement: impl Into<Statement>, action: DbAction) -> Self {
        self.statements.push((statement.into(), action));
        self
    }

    /// Add multiple statements to the step.
    /// # Example
    /// ```
    /// use cortex::objects::step::{Step, StepType};
    /// use cortex::objects::statement::Statement;
    /// use cortex::objects::table::Table;
    /// use cortex::objects::statement::DbAction;
    /// let data = vec![
    ///    (Table::new("test"), DbAction::Create),
    /// ];
    /// let step = Step::new("test", StepType::Update, semver::Version::new(1, 0, 0))
    ///    .add_statements(data);
    /// ```
    pub fn add_statements(mut self, statements: Vec<(impl Into<Statement>, DbAction)>) -> Self {
        self.statements
            .extend(statements.into_iter().map(|(s, a)| (s.into(), a)));
        self
    }

    #[cfg(feature = "postgres")]
    pub fn print_as_pg(&self) -> String {
        use crate::db::producer::postgres::PostgresStatementProducer;

        let mut output = String::new();
        for (statement, action) in &self.statements {
            output.push_str(&PostgresStatementProducer::map(statement, action));
        }
        output
    }
}
