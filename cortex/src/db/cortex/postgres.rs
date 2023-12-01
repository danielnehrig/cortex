use crate::{
    connection::{postgres::Postgres, ExecuteError, ExecuteType},
    db::producer::postgres::PostgresStatementProducer,
    objects::step::Step,
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
}

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
        }
    }

    /// Adds a step to cortex
    pub fn add_step(mut self, step: Step) -> Self {
        self.data.push(step);
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

    /// Executes all steps that have been added to cortex
    pub fn execute(&mut self) -> Result<Self, ExecuteError> {
        if self.data.is_empty() {
            return Err(ExecuteError(
                "no steps have been added to the producer".to_string(),
            ));
        }
        if self
            .data
            .iter()
            .filter(|step| step.version >= self.current_schema_version)
            .count()
            == 0
        {
            return Err(ExecuteError(
                "no steps to update everything on the latest version".to_string(),
            ));
        }
        for step in self.data.clone() {
            if step.version >= self.current_schema_version {
                println!("executing step: {}", step.version);
                match step.s_type {
                    crate::objects::step::StepType::InitSetup => {
                        self.setup_initial_version()?;
                        for (statement, action) in &step.statements {
                            self.connection.execute(ExecuteType::Command(
                                PostgresStatementProducer::map(statement, action),
                            ))?;
                        }
                    }
                    crate::objects::step::StepType::Update => {
                        for (statement, action) in &step.statements {
                            self.connection.execute(ExecuteType::Command(
                                PostgresStatementProducer::map(statement, action),
                            ))?;
                        }
                        self.set_version(&step.version)?;
                    }
                }
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection.clone(),
            current_schema_version: self.current_schema_version.clone(),
            config: self.config.clone(),
        })
    }
}
