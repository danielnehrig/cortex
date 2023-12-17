//! "Objects" are the things that are created in the database.
//! This module contains all of the objects that can be created in the database.

pub mod composite_type;
pub mod database;
pub mod procedure;
pub mod role;
pub mod sequence;
pub mod statement;
pub mod step;
pub mod table;
pub mod trigger;
pub mod user;
pub mod views;

/// The prelude contains all of the objects that can be created in the database.
pub mod prelude {
    pub use super::composite_type::CompositeType;
    pub use super::database::Database;
    pub use super::role::Role;
    pub use super::sequence::Sequence;
    pub use super::statement::{DbAction, Statement};
    pub use super::step::{Step, StepType};
    pub use super::table::{PropAnnotation, PropType, Table, TableField, TableProp};
    pub use super::user::User;
}
