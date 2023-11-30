use cortex::{
    connection::{postgres::Postgres, ConnectionConfig},
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table},
    },
    CortexPostgres,
};

#[warn(dead_code)]
enum Db {
    Postgres,
}

#[cfg(feature = "postgres")]
fn main() {
    use cortex::objects::statement::DbAction;

    let db_in_use = Db::Postgres;
    let users = Table::new("users").add_prop(("id", PropType::Int, None));
    let orders = Table::new("orders").add_prop(("id", PropType::Int, None));
    let db = Database::new("testo");
    let init = Step::new(
        "Init Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(Statement::Database(&db, DbAction::Create));
    let data = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(Statement::Table(&users, DbAction::Create))
    .add_statement(Statement::Table(&orders, DbAction::Create))
    .add_statement(Statement::Table(&users, DbAction::Drop));
    let cleanup = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 3),
    )
    .add_statement(Statement::Database(&db, DbAction::Drop));

    match db_in_use {
        Db::Postgres => {
            let client_conf = ConnectionConfig::<Postgres>::default();
            let cortex_conf = cortex::CortexPostgresConfig {
                plugins: vec![],
                supported_db_versions: (
                    semver::Version::new(15, 0, 0),
                    semver::Version::new(16, 0, 0),
                ),
            };
            let connection = Postgres::new(client_conf).expect("to connect to db");
            let cortex = CortexPostgres::new(connection, cortex_conf)
                .add_step(init)
                .add_step(data)
                .execute()
                .expect("execute to work");
            _ = cortex
                .clean()
                .add_step(cleanup)
                .execute()
                .expect("execute to work");
        }
    }
}
