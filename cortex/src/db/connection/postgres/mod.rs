use postgres::Client;

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

pub struct Postgres(Client);

impl Postgres {
    pub fn new(config: ConnectionConfig<'_, Self>) -> Result<Self, postgres::Error> {
        let uri = config.get_uri();

        let client = Client::connect(&uri, postgres::NoTls)?;

        Ok(Self(client))
    }

    pub fn get_client(&mut self) -> &mut Client {
        &mut self.0
    }
}

impl Postgres {
    pub fn execute(&mut self, data: ExecuteType<'_, Self>) -> Result<(), ExecuteError> {
        match data {
            ExecuteType::Command(command) => self
                .0
                .batch_execute(command.as_str())
                .map_err(|e| ExecuteError(e.to_string()))?,
            ExecuteType::Driver(_) => panic!("c driver based execution not supported"),
        }
        Ok(())
    }
}
