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

#[cfg(feature = "postgres")]
fn main() {
    use cortex::objects::statement::DbAction;

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
    let client_conf = ConnectionConfig::<Postgres>::default();
    let connection = Postgres::new(client_conf).expect("to connect to db");
    let producer = CortexPostgres::new(connection)
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
