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
