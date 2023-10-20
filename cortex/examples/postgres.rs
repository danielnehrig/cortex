use cortex::{
    connection::{postgres::Postgres, ConnectionConfig},
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table, TableProp},
    },
    producer::PostgresStatementProducer,
};

fn main() {
    // don't need to import table prop also can pass slice of tuples with table infos
    let users: Table<PostgresStatementProducer> =
        Table::new("users").add_prop(TableProp::new("id", PropType::Int, None));
    let orders: Table<PostgresStatementProducer> =
        Table::new("orders").add_prop(TableProp::new("id", PropType::Int, None));
    let db: Database<PostgresStatementProducer> = Database::new("test");
    let data = Step::new("Init Schema", StepType::Update)
        .add_statement(Statement::Create(&db))
        .add_statement(Statement::Create(&users))
        .add_statement(Statement::Create(&orders))
        .add_statement(Statement::Drop(&users));
    let _client_conf = ConnectionConfig::<Postgres>::default();
    let producer = PostgresStatementProducer::new().add_step(data);
    println!("{}", producer);
}
