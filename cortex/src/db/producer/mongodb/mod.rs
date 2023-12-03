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
            "$jsonSchema": doc! {
                "bsonType": "object",
                "required": collection.props.iter().map(|p| p.name.to_string()).collect::<Vec<String>>(),
                // create multiple documents from props iter
                "properties": collection.props.iter().fold(doc! {}, |mut acc, p| {
                    let mut prop = doc! {
                        "bsonType": match p.t_type {
                            PropType::Int => "int",
                            PropType::SmallInt => "int",
                            PropType::BigInt => "int",
                            PropType::Double => "double",
                            PropType::Timestamp => "timestamp",
                            PropType::Text => "string",
                            PropType::Date => "date",
                            PropType::Bool => "bool",
                        },
                        "title": p.name.to_string(),
                    };
                    if let Some(annotation) = &p.annotation {
                        match annotation {
                            PropAnnotation::PrimaryKey => {
                                prop.insert("description", "primary key".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Unique => {
                                prop.insert("description", "unique".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::NotNull => {
                                prop.insert("description", "not null".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Default => {
                                prop.insert("description", "default".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Check => {
                                prop.insert("description", "check".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Foreign => {
                                prop.insert("description", "foreign".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Constraint(_) => {
                                prop.insert("description", "constraint".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Empty => {
                                prop.insert("description", "empty".to_string());
                                prop.insert("uniqueItems", true);
                            }
                        }
                    }
                    acc.insert(p.name.to_string(), prop);
                    acc
                }),
            }
        };

        let collection_options = CreateCollectionOptions::builder().validator(schema).build();

        if let Some(session) = session {
            db.create_collection_with_session(&collection.name, collection_options, session)
                .await
                .map_err(|e| {
                    ExecuteError(
                        format!("failed to create collection: {}\n{:#?}", e, collection),
                        collection.database.clone().unwrap_or("".into()).to_string(),
                    )
                })?;
            Ok(())
        } else {
            db.create_collection(&collection.name, collection_options)
                .await
                .map_err(|e| {
                    ExecuteError(
                        format!("failed to create collection: {}\n{:#?}", e, collection),
                        collection.database.clone().unwrap_or("".into()).to_string(),
                    )
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
