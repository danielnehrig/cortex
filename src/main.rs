use cortex::{
    objects::{
        database::Database,
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table, TableProp},
    },
    producer::PostgresStatementProducer,
};

fn main() {
    let users = Table::new("users").add_prop(TableProp::new("id", PropType::Int, None));
    let orders = Table::new("orders").add_prop(TableProp::new("id", PropType::Int, None));
    let db = Database::new("test");
    let data = Step::new("Init Schema", StepType::Update)
        .add_statement(Statement::Create(&db))
        .add_statement(Statement::Create(&users))
        .add_statement(Statement::Create(&orders))
        .add_statement(Statement::Drop(&users));
    let producer = PostgresStatementProducer::new().add_step(data.clone());
    println!("{}", producer);
}
