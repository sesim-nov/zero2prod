use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::AppInfo;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static SUBSCRIBER: Lazy<()> = Lazy::new(|| {
    let name = "Testing".into();
    let level = "debug".into();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(name, level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(name, level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub app_address: String,
    pub db_pool: PgPool,
}

#[tracing::instrument(name = "Spawning Test Server")]
pub async fn spawn_app() -> TestApp {
    // Setup Telemetry (once.)
    Lazy::force(&SUBSCRIBER);

    // Read configuration
    let mut configuration = get_configuration().expect("Failed to get Configuration");
    configuration.database.name = Uuid::new_v4().to_string();
    configuration.app.port = "0".into();

    let db_connection = configure_database(&configuration.database).await;

    // Spawn app
    let app = AppInfo::new(configuration, db_connection.clone()).expect("Failed to build app");
    let _ = tokio::spawn(app.server);

    tracing::info!("App Address: {}", app.app_address);

    TestApp {
        app_address: app.app_address,
        db_pool: db_connection,
    }
}

async fn configure_database(database: &DatabaseSettings) -> PgPool {
    // Acquire a temporary connection to the main database
    let mut tmp_connection = PgConnection::connect_with(&database.without_db())
        .await
        .expect("Temporary connection to main pg database failed.");
    tmp_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, database.name).as_str())
        .await
        .expect("Failed to create database");

    let conn = database.with_db();
    let pool_out = PgPool::connect_with(conn)
        .await
        .expect("Database connection failed.");

    sqlx::migrate!("./migrations")
        .run(&pool_out)
        .await
        .expect("Database migration failed.");

    pool_out
}
