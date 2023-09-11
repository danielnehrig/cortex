#[derive(Debug, Clone, Copy)]
pub enum SupportedDatabases {
    Postgres,
    Mysql,
}

#[derive(Debug, Clone)]
pub struct SchemaClient {
    pub database: SupportedDatabases,
}

impl SchemaClient {
    pub fn new(database: SupportedDatabases) -> Self {
        Self { database }
    }
}
