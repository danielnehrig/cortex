use mongodb::options::TransactionOptions;

use crate::{
    connection::{mongodb::Mongo, ExecuteError, ExecuteType},
    objects::step::Step,
};

#[derive(Debug)]
pub enum ExecutionMode {
    Optimistic,
    Transactional,
}

#[derive(Debug)]
pub struct CortexMongoConfig {
    pub supported_db_versions: (semver::Version, semver::Version),
    pub execution_mode: ExecutionMode,
}

pub struct CortexMongo {
    data: Vec<Step>,
    connection: Mongo,
    config: CortexMongoConfig,
    current_schema_version: semver::Version,
}

impl CortexMongo {
    pub fn new(connection: Mongo, config: CortexMongoConfig) -> Self {
        let current_version = semver::Version::parse("0.0.0").expect("failed to parse version");
        Self {
            data: Vec::new(),
            connection,
            current_schema_version: current_version,
            config,
        }
    }

    pub fn add_step(mut self, step: Step) -> Self {
        self.data.push(step);
        self
    }

    pub fn clean(mut self) -> Self {
        self.data.clear();
        self
    }

    #[allow(dead_code)]
    fn setup_initial_version(&mut self) -> Result<(), ExecuteError> {
        todo!()
    }
    #[allow(dead_code)]
    fn set_version(&mut self, _version: &semver::Version) -> Result<(), ExecuteError> {
        todo!()
    }

    pub async fn execute(self) -> Result<Self, ExecuteError> {
        match self.config.execution_mode {
            ExecutionMode::Optimistic => self.execute_as_optimistic().await,
            // requires mongodb replica set
            ExecutionMode::Transactional => self.execute_as_transaction().await,
        }
    }

    async fn execute_as_optimistic(mut self) -> Result<Self, ExecuteError> {
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
        for step in self.data {
            if step.version >= self.current_schema_version {
                for statement in step.statements {
                    self.connection
                        .execute(ExecuteType::Driver(statement.0, statement.1), None)
                        .await?;
                }
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection,
            current_schema_version: self.current_schema_version.clone(),
            config: self.config,
        })
    }

    async fn execute_as_transaction(mut self) -> Result<Self, ExecuteError> {
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
        for step in self.data {
            if step.version >= self.current_schema_version {
                let mut session = self.connection.0.start_session(None).await.map_err(|e| {
                    ExecuteError(format!("failed to start session for mongodb: {}", e))
                })?;
                let transaction_options = TransactionOptions::builder().build();
                session
                    .start_transaction(transaction_options)
                    .await
                    .map_err(|e| {
                        ExecuteError(format!("failed to start transaction for mongodb: {}", e))
                    })?;
                for statement in step.statements {
                    self.connection
                        .execute(
                            ExecuteType::Driver(statement.0, statement.1),
                            Some(&mut session),
                        )
                        .await?;
                }
                session.commit_transaction().await.map_err(|e| {
                    ExecuteError(format!("failed to commit transaction for mongodb: {}", e))
                })?;
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection,
            current_schema_version: self.current_schema_version.clone(),
            config: self.config,
        })
    }
}
