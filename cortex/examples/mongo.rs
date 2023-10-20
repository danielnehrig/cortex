use cortex::{
    connection::{mongodb::Mongo, ConnectionConfig},
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table, TableProp},
    },
    producer::MongodbStatementProducer,
};

#[derive(Debug)]
struct Products {
    id: i32,
    name: String,
    price: f32,
    quantity: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users = Table::new("users").add_props_as_slice(&[("id", PropType::Int, None)]);
    let orders = Table::new("orders").add_props_as_slice(&[
        ("id", PropType::Int, None),
        ("user_id", PropType::Int, None),
        ("order_date", PropType::Date, None),
    ]);
    let db = Database::new("test");
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
