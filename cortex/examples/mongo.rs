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
    };

    let users: Table = Table::new("users").add_props_as_slice(&[("id", PropType::Int, None)]);
    let orders: Table = Table::new("orders").add_props_as_slice(&[
        ("id", PropType::Int, None),
        ("user_id", PropType::Int, None),
        ("order_date", PropType::Date, None),
    ]);
    let db = Database::new("test");
    let data = Step::new(
        "Init Schema",
        StepType::Update,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(&db, DbAction::Create)
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create)
    .add_statement(&users, DbAction::Drop);
    let client_conf = ConnectionConfig::default();
    let mongo = Mongo::new(client_conf).await?;
    let _ = CortexMongo::new(mongo).add_step(data);
    Ok(())
}
