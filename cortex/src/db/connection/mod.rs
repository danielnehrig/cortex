use crate::objects::statement::{DbAction, Statement};
use thiserror::Error;

#[cfg(feature = "mongodb")]
pub mod mongodb;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub mod prelude {
    #[cfg(feature = "mongodb")]
    pub use super::mongodb::*;
    #[cfg(feature = "postgres")]
    pub use super::postgres::*;
    #[cfg(feature = "sqlite")]
    pub use super::sqlite::*;
    pub use super::ConnectionConfig;
}

#[derive(Debug, Default)]
pub struct ConnectionConfig<'a, T> {
    host: &'a str,
    port: u16,
    username: &'a str,
    password: &'a str,
    database: &'a str,
    path: Option<&'a str>,
    additional: Option<&'a str>,
    marker: std::marker::PhantomData<T>,
}

impl<'a, T> ConnectionConfig<'a, T> {
    pub fn with_host(mut self, host: impl Into<&'a str>) -> Self {
        self.host = host.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_username(mut self, username: impl Into<&'a str>) -> Self {
        self.username = username.into();
        self
    }

    pub fn with_password(mut self, password: impl Into<&'a str>) -> Self {
        self.password = password.into();
        self
    }

    pub fn with_path(mut self, path: impl Into<&'a str>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_additional(mut self, additional: impl Into<&'static str>) -> Self {
        self.additional = Some(additional.into());
        self
    }

    pub fn with_db(mut self, database: impl Into<&'a str>) -> Self {
        self.database = database.into();
        self
    }
}

impl From<ConnectionConfig<'_, postgres::Postgres>> for String {
    fn from(config: ConnectionConfig<'_, postgres::Postgres>) -> Self {
        format!(
            "postgres://{}:{}@{}:{}/{}{}",
            config.username,
            config.password,
            config.host,
            config.port,
            config.database,
            config.path.unwrap_or_default()
        )
    }
}

impl From<ConnectionConfig<'_, mongodb::Mongo>> for String {
    fn from(config: ConnectionConfig<'_, mongodb::Mongo>) -> Self {
        format!(
            "mongodb://{}:{}@{}/{}?{}",
            config.username,
            config.password,
            config.host,
            config.database,
            config.additional.unwrap_or_default()
        )
    }
}

pub enum ExecuteType {
    Command(String),
    Driver(Statement, DbAction),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Database connection error: {0}")]
    ConnectError(#[from] ConnectError),
    #[error("Database execution error: {0}")]
    ExecuteError(#[from] ExecuteError),
    #[error("Database transaction error: {0}")]
    TransactionError(#[from] TransactionError),
    #[error("Database commit error: {0}")]
    CommitError(#[from] CommitError),
    #[error("Database query error: {0}")]
    QueryError(#[from] QueryError),
}

#[derive(Error, Debug)]
#[error("failed to connect to database\n{0}")]
pub struct ConnectError(pub String);

#[derive(Error, Debug)]
#[error("Query execution failed:\n  Query: {0}\n  Error: {1}")]
pub struct ExecuteError(pub String, pub String);

#[derive(Error, Debug)]
#[error("failed to execute query {0}")]
pub struct QueryError(pub String);

#[derive(Error, Debug)]
#[error("failed to start transaction {0}")]
pub struct TransactionError(pub String);

#[derive(Error, Debug)]
#[error("failed to commit {0}")]
pub struct CommitError(pub String);
