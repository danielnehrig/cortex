use mongodb::{bson::doc, ClientSession};
use mongodb::{options::CreateCollectionOptions, Client};

use crate::{
    connection::ExecuteError,
    objects::{
        database::Database,
        statement::DbAction,
        table::{PropAnnotation, PropType, Table, TableProp},
    },
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
        (client, session): (&Client, Option<&mut ClientSession>),
        collection: &Table,
        _action: &DbAction,
    ) -> Result<(), ExecuteError> {
        let db = client.database(collection.database.as_ref().expect("database not set"));
        let schema = doc! {
            "validator": {
                "$jsonSchema": {
                    "bsonType": "object",
                    "required": collection.props.iter().map(|p| p.name.to_string()).collect::<Vec<String>>(),
                    "properties": collection.props.iter().map(|p| {
                        let t = match p.t_type {
                            PropType::Text => "string",
                            PropType::Int => "int",
                            PropType::SmallInt => "int",
                            PropType::BigInt => "int",
                            PropType::Double => "double",
                            PropType::Bool => "bool",
                            PropType::Date => "date",
                            PropType::Timestamp => "timestamp",
                        };
                        let prop = doc! {
                            "bsonType": t,
                        };
                        // no annotations for mongodb
                        let prop = doc! {
                            p.name.to_string(): prop
                        };
                        prop

                    }).collect::<Vec<_>>()
                }
            }
        };

        let collection_options = CreateCollectionOptions::builder()
            .validator(doc! { "$jsonSchema": schema })
            .build();

        if let Some(session) = session {
            db.create_collection_with_session(&collection.name, collection_options, session)
                .await
                .map_err(|e| {
                    ExecuteError(format!(
                        "failed to create collection: {}\n{:#?}",
                        e.to_string(),
                        collection
                    ))
                })?;
            Ok(())
        } else {
            db.create_collection(&collection.name, collection_options)
                .await
                .map_err(|e| {
                    ExecuteError(format!(
                        "failed to create collection: {}\n{:#?}",
                        e.to_string(),
                        collection
                    ))
                })?;
            Ok(())
        }
    }
}

#[allow(dead_code)]
fn table_annotation_to_db(prop: &TableProp) -> String {
    match &prop.annotation.clone() {
        Some(p) => {
            let a = match p {
                PropAnnotation::PrimaryKey => "PRIMARY KEY",
                PropAnnotation::Unique => "UNIQUE",
                PropAnnotation::NotNull => "NOT NULL",
                PropAnnotation::Default => "DEFAULT",
                PropAnnotation::Check => "CHECK",
                PropAnnotation::Foreign => "FOREIGN",
                PropAnnotation::Constraint(_) => "CONSTRAINT",
                PropAnnotation::Empty => "",
            };
            a.to_string()
        }
        None => "".to_string(),
    }
}
