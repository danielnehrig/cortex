use std::rc::Rc;

use crate::{
    connection::{postgres::Postgres, ConnectionError, ExecuteError, ExecuteType},
    db::{
        cortex::{CortexError, ExecutionMode, SchemaVersionError, StepValidationError},
        producer::postgres::PostgresStatementProducer,
    },
    objects::step::Step,
    prelude::StepType,
};

#[derive(Clone)]
pub enum PostgresPlugins {
    /// Postgis is a plugin that allows you to use geospatial data
    Postgis,
    /// Timescale is a plugin that allows you to use time series data in an efficient way
    Timescale,
}

#[derive(Clone)]
pub struct CortexPostgresConfig {
    /// Plugins/Extensions that should be installed on the database
    pub plugins: Vec<PostgresPlugins>,
    /// The supported versions of the database
    pub supported_db_versions: (semver::Version, semver::Version),
    /// The execution mode of cortex
    pub execution_mode: ExecutionMode,
}

type StatementsLen = usize;
type CurrentStatement = usize;
type HookFnParam = (CurrentStatement, StatementsLen);
type HookFn = dyn Fn(HookFnParam);
type Hook = Rc<HookFn>;

#[derive(Clone)]
pub struct CortexPostgres {
    /// The steps that should be executed
    data: Vec<Step>,
    /// The connection to the database
    connection: Postgres,
    /// The config of Cortex
    config: CortexPostgresConfig,
    /// The current version of the database
    current_schema_version: semver::Version,
    /// hooks to run
    after_execute_hooks: Vec<Hook>,
}

impl CortexPostgres {
    /// Creates a new instance of CortexPostgres
    /// Everythig Cortex Prefixed are the main orchestration objects of Cortex
    /// Which are used to create the database and setups around it
    pub fn new(mut connection: Postgres, config: CortexPostgresConfig) -> Self {
        // get the current version of the database
        let mut current_version = semver::Version::parse("0.0.0").expect("failed to parse version");
        let version = connection.query(
            ExecuteType::Command(
                "SELECT version FROM __version__ ORDER BY version DESC LIMIT 1".to_string(),
            ),
            &[],
        );
        if let Ok(version) = version {
            if let Some(version) = version.get(0) {
                current_version =
                    semver::Version::parse(version.get(0)).expect("failed to parse version");
            }
        }
        println!("current version: {}", current_version);
        Self {
            data: Vec::new(),
            connection,
            current_schema_version: current_version,
            config,
            after_execute_hooks: Vec::new(),
        }
    }

    /// Adds a step to cortex
    pub fn add_step(mut self, step: Step) -> Self {
        self.data.push(step);
        self.data.sort_by(|a, b| a.version.cmp(&b.version));
        self
    }

    pub fn add_steps(mut self, steps: Vec<Step>) -> Self {
        self.data.extend(steps);
        self.data.sort_by(|a, b| a.version.cmp(&b.version));
        self
    }

    /// Removes all steps from cortex
    pub fn clean(mut self) -> Self {
        self.data.clear();
        self
    }

    /// private method of db setup for postgres
    /// this is run on InitSetup Step
    fn setup_initial_version(&mut self) -> Result<(), ExecuteError> {
        self.connection.execute(ExecuteType::Command(
            "CREATE TABLE IF NOT EXISTS __version__ (version VARCHAR(255) NOT NULL)".to_string(),
        ))?;
        self.connection.execute(ExecuteType::Command(
            "INSERT INTO __version__ (version) VALUES ('0.0.0')".to_string(),
        ))?;
        self.config.plugins.iter().for_each(|plugin| {
            match plugin {
                PostgresPlugins::Postgis => self.connection.execute(ExecuteType::Command(
                    "CREATE EXTENSION IF NOT EXISTS postgis".to_string(),
                )),
                PostgresPlugins::Timescale => self.connection.execute(ExecuteType::Command(
                    "CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE".to_string(),
                )),
            }
            .expect("failed to create extension");
        });
        Ok(())
    }

    /// if a step is executed update the version of the database
    fn set_version(&mut self, version: &semver::Version) -> Result<(), ExecuteError> {
        self.connection.execute(ExecuteType::Command(format!(
            "INSERT INTO __version__ (version) VALUES ('{}')",
            version
        )))?;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<Self, CortexError> {
        match self.config.execution_mode {
            ExecutionMode::Optimistic => self.execute_as_optimistic(),
            ExecutionMode::Transactional => self.execute_as_transaction(),
        }
    }

    /// Executes all steps that have been added to cortex
    fn execute_as_transaction(&mut self) -> Result<Self, CortexError> {
        if self.data.is_empty() {
            return Err(StepValidationError(
                "no steps have been added to the producer".to_string(),
            ))?;
        }
        if self
            .data
            .iter()
            .filter(|step| step.version > self.current_schema_version)
            .count()
            == 0
        {
            return Err(SchemaVersionError(
                "no steps to update everything on the latest version".to_string(),
            ))?;
        }
        let all_statements_len = self.count_statements();
        for step in self.data.clone() {
            // print version
            println!("version: {} {}", step.version, self.current_schema_version);
            if step.version > self.current_schema_version {
                match step.s_type {
                    StepType::InitSetup => {
                        self.setup_initial_version()
                            .map_err(ConnectionError::ExecuteError)?;
                        for (statement, action) in &step.statements {
                            self.connection
                                .execute(ExecuteType::Command(PostgresStatementProducer::map(
                                    statement, action,
                                )))
                                .map_err(ConnectionError::ExecuteError)?;
                            for hook in &self.after_execute_hooks {
                                hook((0, all_statements_len));
                            }
                        }
                        self.set_version(&step.version)
                            .map_err(ConnectionError::ExecuteError)?;
                    }
                    StepType::Update => {
                        let mut transaction = self
                            .connection
                            .create_transaction()
                            .map_err(ConnectionError::TransactionError)?;
                        for (statement, action) in &step.statements {
                            transaction
                                .execute(ExecuteType::Command(PostgresStatementProducer::map(
                                    statement, action,
                                )))
                                .map_err(ConnectionError::ExecuteError)?;
                            for hook in &self.after_execute_hooks {
                                hook((0, all_statements_len));
                            }
                        }

                        transaction.commit().map_err(ConnectionError::CommitError)?;
                        self.set_version(&step.version)
                            .map_err(ConnectionError::ExecuteError)?;
                    }
                }
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection.clone(),
            current_schema_version: self.current_schema_version.clone(),
            config: self.config.clone(),
            after_execute_hooks: Vec::new(),
        })
    }

    /// Executes all steps that have been added to cortex
    fn execute_as_optimistic(&mut self) -> Result<Self, CortexError> {
        if self.data.is_empty() {
            return Err(StepValidationError(
                "no steps have been added to the producer".to_string(),
            ))?;
        }
        if self
            .data
            .iter()
            .filter(|step| step.version >= self.current_schema_version)
            .count()
            == 0
        {
            return Err(SchemaVersionError(
                "no steps to update everything on the latest version".to_string(),
            ))?;
        }
        let all_statements_len = self.count_statements();
        for step in self.data.clone() {
            if step.version >= self.current_schema_version {
                match step.s_type {
                    StepType::InitSetup => {
                        self.setup_initial_version()
                            .map_err(ConnectionError::ExecuteError)?;
                        for (statement, action) in &step.statements {
                            self.connection
                                .execute(ExecuteType::Command(PostgresStatementProducer::map(
                                    statement, action,
                                )))
                                .map_err(ConnectionError::ExecuteError)?;
                            for hook in &self.after_execute_hooks {
                                hook((0, all_statements_len));
                            }
                        }
                        self.set_version(&step.version)
                            .map_err(ConnectionError::ExecuteError)?;
                    }
                    StepType::Update => {
                        for (statement, action) in &step.statements {
                            self.connection
                                .execute(ExecuteType::Command(PostgresStatementProducer::map(
                                    statement, action,
                                )))
                                .map_err(ConnectionError::ExecuteError)?;
                            for hook in &self.after_execute_hooks {
                                hook((0, all_statements_len));
                            }
                        }
                        self.set_version(&step.version)
                            .map_err(ConnectionError::ExecuteError)?;
                    }
                }
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection.clone(),
            current_schema_version: self.current_schema_version.clone(),
            config: self.config.clone(),
            after_execute_hooks: Vec::new(),
        })
    }

    fn count_statements(&self) -> usize {
        self.data
            .iter()
            .map(|step| step.statements.len())
            .sum::<usize>()
    }
}
