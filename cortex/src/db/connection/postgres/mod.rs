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
pub struct Postgres(pub Rc<RefCell<Client>>);

pub struct PostgresTransaction<'a>(pub postgres::Transaction<'a>);

impl<'a> PostgresTransaction<'a> {
    pub fn execute(&mut self, data: ExecuteType) -> Result<(), ExecuteError> {
        match data {
            ExecuteType::Command(command) => {
                println!("executing command: {}", command);
                return self
                    .0
                    .batch_execute(command.as_str())
                    .map_err(|e| ExecuteError(format!("{} {}", command, e)));
            }
            ExecuteType::Driver(_, _) => panic!("c driver based execution not supported"),
        }
    }

    pub fn commit(self) -> Result<(), ExecuteError> {
        // Check if this is the only reference to the transaction
        self.0.commit().map_err(|e| ExecuteError(e.to_string()))
    }
}

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

    pub fn create_transaction(&mut self) -> Result<PostgresTransaction, ExecuteError> {
        let client = Rc::get_mut(&mut self.0).unwrap();
        let transaction = client
            .get_mut()
            .transaction()
            .map_err(|e| ExecuteError(e.to_string()))?;
        Ok(PostgresTransaction(transaction))
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
            ExecuteType::Driver(_, _) => panic!("c driver based execution not supported"),
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
            ExecuteType::Driver(_, _) => panic!("c driver based execution not supported"),
        }
    }
}
