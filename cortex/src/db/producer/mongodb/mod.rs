mod collection;

use mongodb::Client;
use mongodb::ClientSession;

use crate::{
    connection::ExecuteError,
    db::producer::mongodb::collection::MongodbStatementProducerCollection,
    objects::{database::Database, statement::DbAction, table::Table},
};

pub(crate) struct MongodbStatementProducer;

impl MongodbStatementProducer {
    #[allow(dead_code)]
    pub(crate) fn database_statement(
        (_client, ref mut _session): (&Client, &mut ClientSession),
        _database: &Database,
        _action: &DbAction,
    ) -> Result<(), ExecuteError> {
        unimplemented!("mongodb does not require explicit database creation")
    }

    pub(crate) async fn collection_statement(
        connection: (&Client, Option<&mut ClientSession>),
        collection: &Table,
        action: &DbAction,
    ) -> Result<(), ExecuteError> {
        match action {
            DbAction::Create => {
                MongodbStatementProducerCollection::create_collection(connection, collection).await
            }
            DbAction::Drop => todo!(),
            DbAction::Alter => todo!(),
            DbAction::Insert => todo!(),
            DbAction::Grant => todo!(),
            DbAction::Revoke => todo!(),
        }
    }
}
