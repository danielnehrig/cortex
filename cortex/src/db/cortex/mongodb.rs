use mongodb::options::TransactionOptions;
use tokio::runtime::Runtime;

use crate::{
    connection::{mongodb::Mongo, ExecuteError, ExecuteType},
    objects::step::Step,
};

pub struct CortexMongo {
    data: Vec<Step>,
    connection: Mongo,
    current_schema_version: semver::Version,
}

impl CortexMongo {
    pub fn new(connection: Mongo) -> Self {
        let current_version = semver::Version::parse("0.0.0").expect("failed to parse version");
        Self {
            data: Vec::new(),
            connection,
            current_schema_version: current_version,
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

    pub fn execute(mut self) -> Result<Self, ExecuteError> {
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
        let rt = Runtime::new().unwrap();
        for step in self.data {
            if step.version >= self.current_schema_version {
                rt.block_on(async {
                    let mut session = self.connection.0.start_session(None).await.map_err(|e| {
                        ExecuteError(format!(
                            "failed to start session for mongodb: {}",
                            e.to_string()
                        ))
                    })?;
                    let transaction_options = TransactionOptions::builder().build();
                    session
                        .start_transaction(transaction_options)
                        .await
                        .map_err(|e| {
                            ExecuteError(format!(
                                "failed to start transaction for mongodb: {}",
                                e.to_string()
                            ))
                        })?;
                    for statement in step.statements {
                        self.connection
                            .execute(ExecuteType::Driver(statement.0, statement.1), &mut session)?;
                    }
                    session.commit_transaction().await.map_err(|e| {
                        ExecuteError(format!(
                            "failed to commit transaction for mongodb: {}",
                            e.to_string()
                        ))
                    })?;
                    Ok(())
                })?;
            }
        }
        Ok(Self {
            data: Vec::new(),
            connection: self.connection,
            current_schema_version: self.current_schema_version.clone(),
        })
    }
}
