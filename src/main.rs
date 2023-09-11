use schemacreator::db::{
    objects::{
        statement::Statement,
        step::{Step, StepType},
        table::{PropType, Table, TableProp},
    },
    producer::postgres::PostgresStatementProducer,
};

fn main() {
    let users = Table::<PostgresStatementProducer>::new("users").add_prop(TableProp::new(
        "id",
        PropType::Int,
        None,
    ));
    let orders = Table::new("orders").add_prop(TableProp::new("id", PropType::Int, None));
    let data = Step::new("Init Schema", StepType::Update)
        .add_statement(Statement::Create(&users))
        .add_statement(Statement::Create(&orders))
        .add_statement(Statement::Drop(&users));
    println!("{}", data);
}
