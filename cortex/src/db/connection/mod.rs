use crate::objects::statement::{DbAction, Statement};

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

// create connect error type
#[derive(Debug)]
pub struct ConnectError {
    pub message: String,
}

impl ConnectError {
    pub fn new(message: String) -> Self {
        ConnectError { message }
    }
}

pub enum ExecuteType {
    Command(String),
    Driver(Statement, DbAction),
}

#[derive(Debug)]
pub struct ExecuteError(pub String);

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failed to execute query\n {}", self.0)
    }
}
