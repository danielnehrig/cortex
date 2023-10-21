//! # Cortex
//! Cortex is a library for building database schemas in rust
//! it implements a generic abstraction over x databases
//! which let's you build your schema once and use it with any database.
//! Control the schema of any database with rust ship your product without knowledge
//! of the customers database in advance or tailore your product to your needs instead to the
//! database you are using switch databases without changing your code and figure out what suits
//! your appilcation the best.
//!
//! ## Example
//! ```toml
//! cortex = { git = "https://github.com/danielnehrig/cortex", features = ["postgres"] }
//! ```
//! ```rust
//! use cortex::{
//!     connection::{postgres::Postgres, ConnectionConfig},
//!     objects::{
//!         database::Database,
//!         statement::Statement,
//!         step::{Step, StepType},
//!         table::{PropType, Table, TableProp},
//!     },
//!     producer::PostgresStatementProducer,
//! };
//!
//!     // don't need to import table prop also can pass slice of tuples with table infos
//!     let users: Table<PostgresStatementProducer> = Table::new("users").add_prop(TableProp::new("id", PropType::Int, None));
//!     let orders: Table<PostgresStatementProducer> = Table::new("orders").add_prop(TableProp::new("id", PropType::Int, None));
//!     let db: Database<PostgresStatementProducer> = Database::new("test");
//!     let data = Step::new("Init Schema", StepType::Update)
//!         .add_statement(Statement::Create(&db))
//!         .add_statement(Statement::Create(&users))
//!         .add_statement(Statement::Create(&orders))
//!         .add_statement(Statement::Drop(&users));
//!     let _client_conf = ConnectionConfig::<Postgres>::default();
//!     let producer = PostgresStatementProducer::new().add_step(data);
//!     println!("{}", producer);
//! ```
//! to see more examples take a look into the examples folder
//!
//! further more you can use the `cortex::connection` module to connect to your database
//! we support async implementations and sync implementations
//!
//! ## Features
//! _DB Support_
//! - [x] Postgres
//! - [x] MongoDB
//!
//! <br>
//! _Code  Specific_
//!
//! - [ ] Proc Macro data strucuture generations and struct to table mappin
//! - [ ] Proc Macro Function generation for work with stored procedures
//!   create a stored procedure once tag it and a callable variant will be created as a rust function
//! - [x] Unified API for all databases

mod db;

pub mod objects {
    pub use crate::db::objects::*;
}

pub mod producer {
    #[cfg(feature = "mongodb")]
    pub use crate::db::producer::mongodb::*;
    #[cfg(feature = "postgres")]
    pub use crate::db::producer::postgres::*;
    #[cfg(feature = "sqlite")]
    pub use crate::db::producer::sqlite::*;
}

pub mod connection {
    pub use crate::db::connection::*;
}
