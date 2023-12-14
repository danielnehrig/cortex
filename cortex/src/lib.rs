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
//! ```no_run
//!     use cortex::prelude::*;
//!
//!     // don't need to import table prop also can pass slice of tuples with table infos
//!     let users = Table::new("users").add_prop(("id", PropType::Int32, None));
//!     let orders = Table::new("orders").add_prop(("id", PropType::Int32, None));
//!     let db = Database::new("test");
//!     let data = Step::new("Init Schema", StepType::Update, semver::Version::new(0, 0, 1))
//!         .set_execution_mode(ExecutionMode::Optimistic)
//!         .add_statement(&db, DbAction::Create)
//!         .add_statement(&users, DbAction::Create)
//!         .add_statement(&orders, DbAction::Create)
//!         .add_statement(&users, DbAction::Drop);
//!     let client_conf = ConnectionConfig::<Postgres>::default();
//!     let cortex_conf = CortexPostgresConfig {
//!        plugins: vec![PostgresPlugins::Postgis, PostgresPlugins::Timescale],
//!        supported_db_versions: (
//!            semver::Version::new(15, 0, 0),
//!            semver::Version::new(16, 0, 0),
//!        ),
//!     };
//!     let connection = Postgres::new(client_conf).expect("to connect to db");
//!     let producer = CortexPostgres::new(connection, cortex_conf).add_step(data).execute();
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

#[cfg(feature = "mongodb")]
pub use crate::db::cortex::mongodb::*;
#[cfg(feature = "postgres")]
pub use crate::db::cortex::postgres::*;

pub mod prelude {
    #[cfg(feature = "mongodb")]
    pub use crate::db::cortex::mongodb::*;
    #[cfg(feature = "postgres")]
    pub use crate::db::cortex::postgres::*;
    pub use crate::db::prelude::*;
    pub use crate::objects::prelude::*;
}

pub mod connection {
    pub use crate::db::connection::*;
}
