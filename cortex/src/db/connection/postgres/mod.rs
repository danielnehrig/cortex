use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use postgres::{Client, Row};
use postgres_types::ToSql;

use crate::{
    connection::ExecuteType,
    db::connection::{ConnectionConfig, ExecuteError},
};

impl ConnectionConfig<'_, Postgres> {
    pub fn get_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for ConnectionConfig<'_, Postgres> {
    fn default() -> Self {
        ConnectionConfig {
            host: "localhost",
            port: 5432,
            database: "postgres",
            username: "postgres",
            password: "password",
            path: None,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
/// Postgres connection
/// not thread safe only used to create the db layout with cortex
pub struct Postgres(Rc<RefCell<Client>>);

impl Postgres {
    /// create a new connection
    pub fn new(config: ConnectionConfig<'_, Self>) -> Result<Self, postgres::Error> {
        let uri = config.get_uri();

        let client = Client::connect(&uri, postgres::NoTls)?;

        Ok(Self(Rc::new(RefCell::new(client))))
    }

    pub fn get_client(&mut self) -> RefMut<Client> {
        self.0.borrow_mut()
    }

    /// execute a command
    pub fn execute(&mut self, data: ExecuteType) -> Result<(), ExecuteError> {
        match data {
            ExecuteType::Command(command) => {
                println!("executing command: {}", command);
                return self
                    .0
                    .borrow_mut()
                    .batch_execute(command.as_str())
                    .map_err(|e| ExecuteError(format!("{} {}", command, e)));
            }
            ExecuteType::Driver(_) => panic!("c driver based execution not supported"),
        }
    }

    /// query the database
    pub fn query(
        &mut self,
        data: ExecuteType,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, ExecuteError> {
        match data {
            ExecuteType::Command(command) => self
                .0
                .borrow_mut()
                .query(command.as_str(), params)
                .map_err(|e| ExecuteError(e.to_string())),
            ExecuteType::Driver(_) => panic!("c driver based execution not supported"),
        }
    }
}
