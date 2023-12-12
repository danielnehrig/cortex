use cortex::prelude::*;

#[warn(dead_code)]
enum Db {
    Postgres,
    Mongo,
    All,
}

impl From<String> for Db {
    fn from(db: String) -> Self {
        match db.as_str() {
            "postgres" => Db::Postgres,
            "mongo" => Db::Mongo,
            "all" => Db::All,
            _ => panic!("db not supported"),
        }
    }
}

fn schema_definition() -> Vec<Step> {
    // schema definition
    let users = Table::new("users").add_prop(("id", PropType::Int, None));
    let orders = Table::new("orders").add_prop(("id", PropType::Int, None));
    let db = Database::new("testo");
    let init = Step::new(
        "Init Schema",
        StepType::InitSetup,
        semver::Version::new(0, 0, 1),
    )
    .add_statement(&db, DbAction::Create);
    let data = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 2),
    )
    .add_statement(&users, DbAction::Create)
    .add_statement(&orders, DbAction::Create)
    .add_statement(&users, DbAction::Drop);
    let cleanup = Step::new(
        "Update Schema",
        StepType::Update,
        semver::Version::new(0, 0, 3),
    )
    .add_statement(&db, DbAction::Drop);
    vec![init, data, cleanup]
}

#[tokio::main]
async fn main() {
    let db_in_use: Db = std::env::var("DB").expect("DB env to be set").into();
    let data = schema_definition();

    // execution of schema
    match db_in_use {
        Db::Postgres => {
            let client_conf = ConnectionConfig::<Postgres>::default();
            let cortex_conf = cortex::CortexPostgresConfig {
                plugins: vec![],
                supported_db_versions: (
                    semver::Version::new(15, 0, 0),
                    semver::Version::new(16, 0, 0),
                ),
            };
            let connection = Postgres::new(client_conf).expect("to connect to db");
            let _ = CortexPostgres::new(connection, cortex_conf)
                .add_steps(data)
                .execute()
                .expect("execute to work");
        }
        Db::Mongo => {
            let client_conf = ConnectionConfig::<Mongo>::default();
            let cortex_conf = cortex::CortexMongoConfig {
                supported_db_versions: (
                    semver::Version::new(15, 0, 0),
                    semver::Version::new(16, 0, 0),
                ),
            };
            let connection = Mongo::new(client_conf).await.expect("to connect to db");
            let _ = CortexMongo::new(connection, cortex_conf)
                .add_steps(data)
                .execute()
                .await
                .expect("execute to work");
        }
        Db::All => todo!(),
    }
}
