use cortex::prelude::*;

#[cfg(feature = "postgres")]
fn main() {
    let db = Database::new("testo");
    let users = Table::new("users").add_prop(("id", PropType::Int, None));
    let orders = Table::new("orders").add_prop(("id", PropType::Int, None));
    let init = Step::new(
        "Init Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(&db, DbAction::Create);
    let data = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create)
    .add_statement(&users, DbAction::Drop);
    let cleanup = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 3),
    )
    .add_statement(&db, DbAction::Drop);
    let client_conf = ConnectionConfig::<Postgres>::default();
    let cortex_conf = CortexPostgresConfig {
        plugins: vec![PostgresPlugins::Postgis, PostgresPlugins::Timescale],
        supported_db_versions: (
            semver::Version::new(15, 0, 0),
            semver::Version::new(16, 0, 0),
        ),
    };
    let connection = Postgres::new(client_conf).expect("to connect to db");
    let producer = CortexPostgres::new(connection, cortex_conf)
        .add_step(init)
        .add_step(data)
        .execute()
        .expect("execute to work");
    _ = producer
        .clean()
        .add_step(cleanup)
        .execute()
        .expect("execute to work");
}
