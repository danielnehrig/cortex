use cortex::{
    objects::{
        database::Database,
        step::{Step, StepType},
        table::{PropType, Table},
    },
    CortexMongo,
};

#[cfg(feature = "mongodb")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cortex::{
        connection::{mongodb::Mongo, ConnectionConfig},
        objects::statement::DbAction,
        CortexMongoConfig, ExecutionMode,
    };

    let users: Table = Table::new("users")
        .add_props_as_slice(&[("id", PropType::Int, None)])
        .on_db("test");
    let orders: Table = Table::new("orders")
        .add_props_as_slice(&[
            ("id", PropType::Int, None),
            ("user_id", PropType::Int, None),
            ("order_date", PropType::Date, None),
        ])
        .on_db("admin");
    let db = Database::new("test");
    let data = Step::new(
        "Init Schema",
        StepType::Update,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(db, DbAction::Create)
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create)
    .add_statement(&users, DbAction::Drop);
    let client_conf = ConnectionConfig::default();
    let mongo = Mongo::new(client_conf).await.expect("Failed to connect");
    let cortex_config = CortexMongoConfig {
        supported_db_versions: (semver::Version::new(0, 0, 1), semver::Version::new(0, 0, 1)),
        execution_mode: ExecutionMode::Optimistic,
    };
    let cortex = CortexMongo::new(mongo, cortex_config).add_step(data);
    _ = cortex.execute().await.expect("Failed to execute");
    Ok(())
}
