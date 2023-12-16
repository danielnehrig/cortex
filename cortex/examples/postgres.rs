use anyhow::Result;
use cortex::prelude::*;

#[cfg(feature = "postgres")]
fn main() -> Result<()> {
    let ts_db = Database::new("timeseries");
    let config_db = Database::new("config");
    let sales_db = Database::new("sales");

    let users_table = Table::new("users")
        .add_prop_with_annotation(("id", PropType::Int32, PropAnnotation::PrimaryKey))
        .on_db(&config_db);
    let products_table = Table::new("products")
        .add_prop_with_annotation(("id", PropType::Int32, PropAnnotation::PrimaryKey))
        .add_prop_with_annotation(("name", PropType::Text, PropAnnotation::NotNull))
        .add_prop_with_annotation(("price", PropType::Double, PropAnnotation::NotNull))
        .add_prop_with_annotation(("quantity", PropType::Int32, PropAnnotation::NotNull))
        .on_db(&config_db);
    let orders_table = Table::new("orders")
        .add_prop_with_annotation(("id", PropType::Int32, PropAnnotation::PrimaryKey))
        .add_foreign_key("id_users", PropType::Int32, &users_table)
        .on_db(&config_db);
    let login_table = Table::new("login")
        .add_prop_with_annotation(("id", PropType::Int32, PropAnnotation::PrimaryKey))
        .on_db(&ts_db);
    let earnings_table = Table::new("earnings")
        .add_prop_with_annotation(("id", PropType::Int32, PropAnnotation::PrimaryKey))
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
    .add_statement(&products_table, DbAction::Create)
    .add_statement(&users_table, DbAction::Create)
    .add_statement(&orders_table, DbAction::Create)];
    let sales_db_steps = vec![Step::new(
        "Sales Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .set_execution_mode(ExecutionMode::Transactional)
    .add_statement(&earnings_table, DbAction::Create)];
    let data_db_step = Step::new(
        "Timeseries Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&login_table, DbAction::Create);
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
        supported_db_versions: (
            semver::Version::new(15, 0, 0),
            semver::Version::new(16, 0, 0),
        ),
    };

    let global_connection = Postgres::new(global_connection_config)?;
    CortexPostgres::new(global_connection, cortex_conf.clone())
        .add_step(global_db_step)
        .execute()?;

    let ts_connection = Postgres::new(ts_connection_config)?;
    let config_connection = Postgres::new(config_connection_config)?;
    let sales_connection = Postgres::new(sales_connection_config)?;

    CortexPostgres::new(ts_connection, cortex_conf.clone())
        .add_step(empty_init.clone())
        .add_step(data_db_step)
        .execute()?;
    CortexPostgres::new(config_connection, cortex_conf.clone())
        .add_step(empty_init.clone())
        .add_steps(conf_db_steps)
        .execute()?;
    CortexPostgres::new(sales_connection, cortex_conf)
        .add_step(empty_init)
        .add_steps(sales_db_steps)
        .execute()?;
    Ok(())
}
