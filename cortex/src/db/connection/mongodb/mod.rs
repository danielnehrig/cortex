use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, ClientSession,
};

use crate::{
    connection::{ExecuteError, ExecuteType},
    db::{connection::ConnectionConfig, producer::mongodb::MongodbStatementProducer},
    objects::statement::Statement,
};

impl ConnectionConfig<'_, Mongo> {
    pub fn get_uri(&self) -> String {
        // this is wont allow transaction since no replica set
        // mongodb://root:example@localhost:27017/admin?authSource=admin&retryWrites=true
        format!(
            "mongodb://{}:{}@{}:{}/{}?authSource=admin&retryWrites=true",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for ConnectionConfig<'_, Mongo> {
    fn default() -> Self {
        ConnectionConfig {
            username: "root",
            password: "example",
            host: "localhost",
            port: 27017,
            database: "default",
            marker: std::marker::PhantomData,
            path: None,
        }
    }
}

pub struct Mongo(pub Client);

impl Mongo {
    pub async fn execute(
        &mut self,
        data: ExecuteType,
        session: Option<&mut ClientSession>,
    ) -> Result<(), ExecuteError> {
        match data {
            ExecuteType::Command(_) => {
                panic!("mongodb does not work like sql we can not execute command directly afaik")
            }
            ExecuteType::Driver(statement, action) => match statement {
                Statement::Table(t) => {
                    MongodbStatementProducer::collection_statement((&self.0, session), &t, &action)
                        .await
                }
                Statement::Database(_) => {
                    // MongodbStatementProducer::database_statement(&self.0, &d, &action)
                    // do nothing
                    Ok(())
                }
            },
        }
    }

    #[cfg(feature = "async")]
    pub async fn new(config: ConnectionConfig<'_, Mongo>) -> mongodb::error::Result<Self> {
        // Replace the placeholder with your Atlas connection string

        use mongodb::bson::doc;
        let uri = config.get_uri();
        let mut client_options = ClientOptions::parse(uri).await?;

        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        // Create a new client and connect to the server
        let client = Client::with_options(client_options)?;

        // Send a ping to confirm a successful connection
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");

        Ok(Self(client))
    }

    #[cfg(feature = "async")]
    pub async fn get_client(&self) -> Client {
        self.0.clone()
    }

    #[cfg(feature = "async")]
    pub async fn get_database(&self, database_name: &str) -> mongodb::Database {
        self.0.database(database_name)
    }
}
