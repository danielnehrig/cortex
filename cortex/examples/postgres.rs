use anyhow::Result;
use cortex::prelude::*;

#[cfg(feature = "postgres")]
fn main() -> Result<()> {
    let ts_db = Database::new("timeseries");
    let config_db = Database::new("config");
    let sales_db = Database::new("sales");

    let users = Table::new("users")
        .add_prop(("id", PropType::Int, None))
        .on_db(&config_db);
    let orders = Table::new("orders")
        .add_prop(("id", PropType::Int, None))
        .on_db(&sales_db);
    let data = Table::new("data")
        .add_prop(("id", PropType::Int, None))
        .on_db(&ts_db);
    let earnings = Table::new("earnings")
        .add_prop(("id", PropType::Int, None))
        .on_db(&sales_db);

    let global_db_step = Step::new(
        "Init Global Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(&ts_db, DbAction::Create)
    .add_statement(&config_db, DbAction::Create)
    .add_statement(&sales_db, DbAction::Create);

    let conf_db_steps = vec![Step::new(
        "Config Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create)];
    let sales_db_steps = vec![Step::new(
        "Sales Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&earnings, DbAction::Create)];
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

    let global_connection_config = ConnectionConfig::<Postgres>::default();
    let ts_connection_config = ConnectionConfig::<Postgres>::default().with_db(&ts_db);
    let config_connection_config = ConnectionConfig::<Postgres>::default().with_db(&config_db);
    let sales_connection_config = ConnectionConfig::<Postgres>::default().with_db(&sales_db);
    let cortex_conf = CortexPostgresConfig {
        plugins: vec![PostgresPlugins::Postgis, PostgresPlugins::Timescale],
        execution_mode: ExecutionMode::Transactional,
        supported_db_versions: (
            semver::Version::new(15, 0, 0),
            semver::Version::new(16, 0, 0),
        ),
    };

    let global_connection = Postgres::new(global_connection_config)?;
    let _ = CortexPostgres::new(global_connection, cortex_conf.clone())
        .add_step(global_db_step)
        .execute()?;

    let ts_connection = Postgres::new(ts_connection_config)?;
    let config_connection = Postgres::new(config_connection_config)?;
    let sales_connection = Postgres::new(sales_connection_config)?;

    let _ = CortexPostgres::new(ts_connection, cortex_conf.clone())
        .add_step(empty_init.clone())
        .add_step(data_db_step)
        .execute()?;
    let _ = CortexPostgres::new(config_connection, cortex_conf.clone())
        .add_step(empty_init.clone())
        .add_steps(conf_db_steps)
        .execute()?;
    let _ = CortexPostgres::new(sales_connection, cortex_conf)
        .add_step(empty_init)
        .add_steps(sales_db_steps)
        .execute()?;
    Ok(())
}
