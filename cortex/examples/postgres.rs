use cortex::{
    connection::{postgres::Postgres, ConnectionConfig},
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table},
    },
    producer::PostgresStatementProducer,
};

#[cfg(feature = "postgres")]
fn main() {
    let users: Table<PostgresStatementProducer> =
        Table::new("users").add_prop(("id", PropType::Int, None));
    let orders: Table<PostgresStatementProducer> =
        Table::new("orders").add_prop(("id", PropType::Int, None));
    let db: Database<PostgresStatementProducer> = Database::new("testo");
    let init = Step::new(
        "Init Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(Statement::Create(&db));
    let data = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(Statement::Create(&users))
    .add_statement(Statement::Create(&orders))
    .add_statement(Statement::Drop(&users));
    let cleanup = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 3),
    )
    .add_statement(Statement::Drop(&db));
    let client_conf = ConnectionConfig::<Postgres>::default();
    let connection = Postgres::new(client_conf).expect("to connect to db");
    let mut producer = PostgresStatementProducer::new(connection)
        .add_step(init)
        .add_step(data)
        .execute()
        .expect("execute to work");
    println!("{}", producer);
    producer = producer
        .clean()
        .add_step(cleanup)
        .execute()
        .expect("execute to work");
    println!("{}", producer);
}
