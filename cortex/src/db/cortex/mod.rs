use thiserror::Error;

use crate::connection::ConnectionError;

pub mod mongodb;
pub mod postgres;

pub mod prelude {
    #[cfg(feature = "mongodb")]
    pub use super::mongodb::*;
    #[cfg(feature = "postgres")]
    pub use super::postgres::*;
    pub use super::ExecutionMode;
}

#[derive(Clone, Debug)]
pub enum ExecutionMode {
    Optimistic,
    Transactional,
}

#[derive(Error, Debug)]
pub enum CortexError {
    #[error("Cortex Database error: {0}")]
    Database(#[from] ConnectionError),
    #[error("Cortex step error: {0}")]
    StepValidation(#[from] StepValidationError),
    #[error("Cortex schema error: {0}")]
    SchemaVersion(#[from] SchemaVersionError),
}

#[derive(Error, Debug)]
#[error("validation failed {0}")]
pub struct StepValidationError(pub String);

#[derive(Error, Debug)]
#[error("schema version error {0}")]
pub struct SchemaVersionError(pub String);
