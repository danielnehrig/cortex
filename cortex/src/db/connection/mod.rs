use crate::objects::statement::Statement;

#[cfg(feature = "mongodb")]
pub mod mongodb;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

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

pub enum ExecuteType<'a, T> {
    Command(String),
    Driver(Statement<'a, T>),
}

#[derive(Debug)]
pub struct ExecuteError(pub String);

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failed to execute query\n {}", self.0)
    }
}
