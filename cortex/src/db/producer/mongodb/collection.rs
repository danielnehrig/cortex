use mongodb::{bson::doc, ClientSession};
use mongodb::{options::CreateCollectionOptions, Client};

use crate::{
    connection::ExecuteError,
    objects::table::{PropAnnotation, PropType, Table},
};

pub struct MongodbStatementProducerCollection;

impl MongodbStatementProducerCollection {
    pub(super) async fn create_collection(
        (client, session): (&Client, Option<&mut ClientSession>),
        collection: &Table,
    ) -> Result<(), ExecuteError> {
        let db = client.database(collection.database.as_ref().expect("database not set"));
        let schema = doc! {
            "$jsonSchema": doc! {
                "bsonType": "object",
                "required": collection.props.iter().map(|p| p.field.get_text()).collect::<Vec<String>>(),
                // create multiple documents from props iter
                "properties": collection.props.iter().fold(doc! {}, |mut acc, p| {
                    let mut prop = doc! {
                        "bsonType": match p.field_type {
                            PropType::Int8 => "int",
                            PropType::Int16 => "int",
                            PropType::Int32 => "int",
                            PropType::Int64 => "int",
                            PropType::UInt8 => "int",
                            PropType::UInt16 => "int",
                            PropType::UInt32 => "int",
                            PropType::UInt64 => "int",
                            PropType::SmallInt => "int",
                            PropType::BigInt => "int",
                            PropType::Double => "double",
                            PropType::Timestamp => "timestamp",
                            PropType::Text => "string",
                            PropType::Date => "date",
                            PropType::Bool => "bool",
                        },
                        "title": p.field.get_text(),
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
                            PropAnnotation::ForeignKey(_) => {
                                prop.insert("description", "foreign".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            PropAnnotation::Constraint(_) => {
                                prop.insert("description", "constraint".to_string());
                                prop.insert("uniqueItems", true);
                            }
                            _ => {}
                        }
                    }
                    acc.insert(p.field.get_text(), prop);
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
