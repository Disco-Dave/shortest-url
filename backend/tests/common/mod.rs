use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use shortest_url::settings::{DatabaseSettings, Settings};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn create_database(settings: &DatabaseSettings) -> PgPool {
    let database_name = format!("test_{}_{}", settings.database_name, Uuid::new_v4());

    let create_conn_string = format!(
        "postgresql://{}:{}@{}:{}",
        settings.migration_username, settings.migration_password, settings.host, settings.port,
    );

    let mut create_conn = PgConnection::connect(&create_conn_string)
        .await
        .expect("Failed to connect to Postgres for database creation");

    create_conn
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, database_name))
        .await
        .expect(&format!(
            "Failed to create test database: {}",
            database_name
        ));

    let migrate_conn_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        settings.migration_username,
        settings.migration_password,
        settings.host,
        settings.port,
        database_name
    );

    let migrate_pool = PgPool::connect(&migrate_conn_string)
        .await
        .expect("Failed to construct connection pool for migration");

    sqlx::migrate!("./migrations")
        .run(&migrate_pool)
        .await
        .expect("Failed to migrate the database");

    PgPool::connect(&settings.app_conn_string())
        .await
        .expect("Failed to construct connection pool")
}

pub async fn spawn_app() -> TestApp {
    let settings = {
        let mut settings = Settings::new().expect("Failed to read configuration.");
        settings.http.port = 0;
        settings
    };

    let db_pool = create_database(&settings.database).await;
    let (addr, server) = shortest_url::start(&settings.http, &settings.database).await;

    tokio::spawn(server);

    let address = format!("http://{}:{}", &settings.http.host, addr.port());

    TestApp { address, db_pool }
}
