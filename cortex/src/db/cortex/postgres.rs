use crate::{
    connection::{postgres::Postgres, ExecuteError, ExecuteType},
    db::producer::postgres::PostgresStatementProducer,
    objects::step::Step,
};

#[derive(Clone)]
pub struct CortexPostgres<'a> {
    data: Vec<Step<'a>>,
    connection: Postgres,
    current_version: semver::Version,
}

impl<'a> CortexPostgres<'a> {
    pub fn new(mut connection: Postgres) -> Self {
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
            current_version,
        }
    }

    pub fn add_step(mut self, step: Step<'a>) -> Self {
        self.data.push(step);
        self.data.sort_by(|a, b| a.version.cmp(&b.version));
        self
    }

    pub fn clean(mut self) -> Self {
        self.data.clear();
        self
    }

    fn setup_initial_version(&mut self) -> Result<(), ExecuteError> {
        self.connection.execute(ExecuteType::Command(
            "CREATE TABLE IF NOT EXISTS __version__ (version VARCHAR(255) NOT NULL)".to_string(),
        ))?;
        self.connection.execute(ExecuteType::Command(
            "INSERT INTO __version__ (version) VALUES ('0.0.0')".to_string(),
        ))?;
        self.connection.execute(ExecuteType::Command(
            "CREATE EXTENSION IF NOT EXISTS postgis".to_string(),
        ))?;
        self.connection.execute(ExecuteType::Command(
            "CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE".to_string(),
        ))?;
        Ok(())
    }

    fn set_version(&mut self, version: &semver::Version) -> Result<(), ExecuteError> {
        self.connection.execute(ExecuteType::Command(format!(
            "INSERT INTO __version__ (version) VALUES ('{}')",
            version
        )))?;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<Self, ExecuteError> {
        if self.data.is_empty() {
            return Err(ExecuteError(
                "no steps have been added to the producer".to_string(),
            ));
        }
        if self
            .data
            .iter()
            .filter(|step| step.version >= self.current_version)
            .count()
            == 0
        {
            return Err(ExecuteError(
                "no steps to update everything on the latest version".to_string(),
            ));
        }
        for step in self.data.clone() {
            if step.version >= self.current_version {
                println!("executing step: {}", step.version);
                match step.s_type {
                    crate::objects::step::StepType::InitSetup => {
                        self.setup_initial_version()?;
                        for statement in &step.statements {
                            self.connection.execute(ExecuteType::Command(
                                PostgresStatementProducer::map(statement),
                            ))?;
                        }
                    }
                    crate::objects::step::StepType::Update => {
                        for statement in &step.statements {
                            self.connection.execute(ExecuteType::Command(
                                PostgresStatementProducer::map(statement),
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
            current_version: self.current_version.clone(),
        })
    }
}
