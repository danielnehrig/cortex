use crate::db::connection::ConnectionConfig;

impl ConnectionConfig<'_, SQLite> {
    pub fn get_uri(&self) -> String {
        format!("sqlite://{}", self.path.unwrap())
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
