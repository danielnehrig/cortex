// use cortex::prelude::*;

// #[cfg(feature = "sqlite")]
// fn main() {
// // don't need to import table prop also can pass slice of tuples with table infos

// use cortex::objects::table::{PropType, Table};
// let users: Table<SQLiteStatementProducer> =
// Table::new("users").add_prop(("id", PropType::Int, None));
// let orders: Table<SQLiteStatementProducer> =
// Table::new("orders").add_prop(("id", PropType::Int, None));
// let db: Database<SQLiteStatementProducer> = Database::new("test");
// let init = Step::new(
// "Init Schema",
// StepType::InitSetup,
// semver::Version::new(0, 0, 1),
// )
// .add_statement(Statement::Create(&db))
// .add_statement(Statement::Create(&users))
// .add_statement(Statement::Create(&orders))
// .add_statement(Statement::Drop(&users));
// let _client_conf = ConnectionConfig::<SQLite>::default();
// let producer = SQLiteStatementProducer::new().add_step(init);
// println!("{}", producer);
// }
fn main() {}
