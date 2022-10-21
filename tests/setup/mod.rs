use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::mail::EmailClient;
use zero2prod::startup::run;
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

pub async fn spawn_app() -> TestApp {
    // Setup Telemetry (once.)
    Lazy::force(&SUBSCRIBER);

    // Read configuration
    let mut configuration = get_configuration().expect("Failed to get Configuration");
    configuration.database.name = Uuid::new_v4().to_string();

    // Connect to db
    let db_connection = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test port");
    let port = listener.local_addr().unwrap().port();

    // Configure email client
    let sender = configuration
        .email_client
        .sender()
        .expect("Failed to parse sender email");
    let email_client = EmailClient::new(sender, configuration.email_client.base_url);

    // Spawn app
    let app = run(listener, db_connection.clone(), email_client).expect("Failed to spawn server");
    let _ = tokio::spawn(app);
    TestApp {
        app_address: format!("http://127.0.0.1:{}", port),
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
