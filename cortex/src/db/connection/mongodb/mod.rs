use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use crate::db::connection::ConnectionConfig;

impl ConnectionConfig<'_> {
    pub fn get_uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for ConnectionConfig<'_> {
    fn default() -> Self {
        Self {
            username: "root",
            password: "example",
            host: "localhost",
            port: 27017,
            database: "test",
        }
    }
}

pub struct Mongo(Client);

impl Mongo {
    #[cfg(feature = "async")]
    pub async fn new(config: ConnectionConfig<'_>) -> mongodb::error::Result<Self> {
        // Replace the placeholder with your Atlas connection string
        let uri = config.get_uri();
        let mut client_options = ClientOptions::parse(uri).await?;

        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        // Create a new client and connect to the server
        let client = Client::with_options(client_options)?;

        // Send a ping to confirm a successful connection
        // client
        // .database("admin")
        // .run_command(doc! {"ping": 1}, None)
        // .await?;
        // println!("Pinged your deployment. You successfully connected to MongoDB!");

        Ok(Self(client))
    }

    #[cfg(feature = "async")]
    pub async fn get_client(&self) -> mongodb::error::Result<Client> {
        Ok(self.0.clone())
    }

    #[cfg(feature = "async")]
    pub async fn get_database(
        &self,
        database_name: &str,
    ) -> mongodb::error::Result<mongodb::Database> {
        Ok(self.0.database(database_name))
    }
}
