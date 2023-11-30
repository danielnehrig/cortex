use mongodb::Client;
use tokio::runtime::Runtime;

use crate::{connection::ExecuteError, objects::statement::DbAction};

pub(crate) struct MongodbStatementProducer;

impl MongodbStatementProducer {
    pub(crate) fn database_statement(
        client: &Client,
        database: &str,
        action: &DbAction,
    ) -> Result<(), ExecuteError> {
        let rt = Runtime::new().unwrap();

        match action {
            DbAction::Create => rt.block_on(async {
                client
                    .database(database)
                    .create_collection("db_created", None)
                    .await
                    .map_err(|e| ExecuteError(e.to_string()))
            }),
            DbAction::Drop => rt.block_on(async {
                client
                    .database(database)
                    .drop(None)
                    .await
                    .map_err(|e| ExecuteError(e.to_string()))
            }),
            DbAction::Alter => panic!("altering a database is not supported"),
            DbAction::Insert => panic!("inserting a database is not supported"),
        }
    }

    // fn create_collection(
    // client: &Client,
    // collection: &Table,
    // action: &DbAction,
    // ) -> Result<(), ExecuteError> {
    // let db = client.database(&collection.database);

    // let rt = Runtime::new().unwrap();

    // rt.block_on(async {
    // return db
    // .create_collection(&collection.name, None)
    // .await
    // .map_err(|e| ExecuteError(e.to_string()));
    // });
    // Ok(())
    // }
}
