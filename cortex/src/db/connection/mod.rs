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
    marker: std::marker::PhantomData<T>,
}

impl<'a, T> ConnectionConfig<'a, T> {
    pub fn with_db(mut self, db: impl Into<&'a str>) -> Self {
        self.database = db.into();
        self
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
