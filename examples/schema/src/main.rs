use anyhow::Result;
use cortex::prelude::*;

fn main() -> Result<()> {
    let data = schema::schema();
    let global_connection_config = ConnectionConfig::<Postgres>::default();
    let ts_connection_config = ConnectionConfig::<Postgres>::default().with_db(&data.0[0]);
    let config_connection_config = ConnectionConfig::<Postgres>::default().with_db(&data.0[1]);
    let sales_connection_config = ConnectionConfig::<Postgres>::default().with_db(&data.0[2]);
    let cortex_conf = CortexPostgresConfig {
        plugins: vec![PostgresPlugins::Postgis, PostgresPlugins::Timescale],
        supported_db_versions: (
            semver::Version::new(15, 0, 0),
            semver::Version::new(16, 0, 0),
        ),
    };

    let global_connection = Postgres::new(global_connection_config)?;
    CortexPostgres::new(global_connection, cortex_conf.clone())
        .add_step(data.1[0].clone())
        .execute()?;

    let ts_connection = Postgres::new(ts_connection_config)?;
    let config_connection = Postgres::new(config_connection_config)?;
    let sales_connection = Postgres::new(sales_connection_config)?;

    CortexPostgres::new(ts_connection, cortex_conf.clone())
        .add_step(data.1[5].clone())
        .add_step(data.1[4].clone())
        .execute()?;
    CortexPostgres::new(config_connection, cortex_conf.clone())
        .add_step(data.1[5].clone())
        .add_step(data.1[1].clone())
        .execute()?;
    CortexPostgres::new(sales_connection, cortex_conf)
        .add_step(data.1[5].clone())
        .add_step(data.1[2].clone())
        .execute()?;
    Ok(())
}
