use crate::db::connection::ConnectionConfig;

impl ConnectionConfig<'_, SQLite> {
    pub fn get_uri(&self) -> String {
        format!("sqlite://{}", self.path.unwrap())
    }
}

impl Default for ConnectionConfig<'_, SQLite> {
    fn default() -> Self {
        ConnectionConfig {
            username: "",
            password: "",
            host: "",
            port: 0,
            database: "",
            marker: std::marker::PhantomData,
            additional: None,
            path: Some("test.db"),
        }
    }
}

pub struct SQLite(sqlite::Connection);

impl SQLite {
    pub fn new(cfg: ConnectionConfig<Self>) -> Self {
        Self(sqlite::Connection::open(cfg.path.unwrap()).unwrap())
    }

    pub fn get_connection(&self) -> &sqlite::Connection {
        &self.0
    }
}
