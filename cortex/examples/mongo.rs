use cortex::{
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table},
    },
    producer::MongodbStatementProducer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users: Table<MongodbStatementProducer> =
        Table::new("users").add_props_as_slice(&[("id", PropType::Int, None)]);
    let orders: Table<MongodbStatementProducer> = Table::new("orders").add_props_as_slice(&[
        ("id", PropType::Int, None),
        ("user_id", PropType::Int, None),
        ("order_date", PropType::Date, None),
    ]);
    let db: Database<MongodbStatementProducer> = Database::new("test");
    let data = Step::new("Init Schema", StepType::Update)
        .add_statement(Statement::Create(&db))
        .add_statement(Statement::Create(&users))
        .add_statement(Statement::Create(&orders))
        .add_statement(Statement::Drop(&users));
    // let client_conf = ConnectionConfig::default();
    // let mongo = Mongo::new(client_conf).await?;
    // let client = mongo.get_client().await?;
    let producer = MongodbStatementProducer::new().add_step(data);
    println!("{}", producer);
    Ok(())
}
