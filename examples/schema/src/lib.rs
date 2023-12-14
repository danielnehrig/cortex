use cortex::prelude::*;
pub fn schema() -> (Vec<Database>, Vec<Step>) {
    let ts_db = Database::new("timeseries");
    let config_db = Database::new("config");
    let sales_db = Database::new("sales");

    let users = Table::new("users")
        .add_prop("id", PropType::Int32)
        .add_prop("name", PropType::Text)
        .add_prop("name2", PropType::Text)
        .on_db(&config_db);
    let orders = Table::new("orders")
        .add_prop("id", PropType::Int32)
        .add_prop("name", PropType::Text)
        .on_db(&sales_db);
    let data = Table::new("data")
        .add_prop("id", PropType::Int32)
        .on_db(&ts_db);
    let earnings = Table::new("earnings")
        .add_prop("id", PropType::Int32)
        .add_prop("dope", PropType::Text)
        .on_db(&sales_db);

    let global_db_step = Step::new(
        "Init Global Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(&ts_db, DbAction::Create)
    .add_statement(&config_db, DbAction::Create)
    .add_statement(&sales_db, DbAction::Create);

    let conf_db_steps = Step::new(
        "Config Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create);
    let sales_db_steps = Step::new(
        "Sales Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .set_execution_mode(ExecutionMode::Transactional)
    .add_statement(&earnings, DbAction::Create);
    let data_db_step = Step::new(
        "Timeseries Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&data, DbAction::Create);
    let empty_init = Step::new(
        "Init Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    );
    (
        vec![ts_db, config_db, sales_db],
        vec![
            global_db_step,
            conf_db_steps,
            sales_db_steps,
            data_db_step,
            empty_init,
        ],
    )
}
