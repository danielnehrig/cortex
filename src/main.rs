use schemacreator::db::{
    objects::{
        statement::{CreateObject, Statement},
        step::{Step, StepType},
        table::{PropType, Table, TableProp},
    },
    producer::{postgres::PostgresStatementProducer, StatementProducer},
};

fn main() {
    let users = Table::<PostgresStatementProducer>::new("users").add_prop(TableProp::new(
        "id",
        PropType::Int,
        None,
    ));
    let orders = Table::new("orders").add_prop(TableProp::new("id", PropType::Int, None));
    let data = Step::new("Init Schema", StepType::Update)
        .add_statement(Statement::Create(CreateObject::Table(users.clone())))
        .add_statement(Statement::Create(CreateObject::Table(orders)));
    let test = PostgresStatementProducer {}.create_table(&users);
    println!("{:?}", data);
    println!("{:?}", test);
}
