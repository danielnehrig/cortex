#[cfg(feature = "mongodb")]
pub mod mongodb;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[derive(Debug)]
pub struct ConnectionConfig<'a, T> {
    host: &'a str,
    port: u16,
    username: &'a str,
    password: &'a str,
    database: &'a str,
    marker: std::marker::PhantomData<T>,
}
