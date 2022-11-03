use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
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
    pub app_port: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    #[tracing::instrument(name = "Spawning Test Server")]
    pub async fn spawn_new() -> TestApp {
        // Setup Telemetry (once.)
        Lazy::force(&SUBSCRIBER);

        // Start mock email server
        let email_server = MockServer::start().await;

        // Read configuration
        let configuration = {
            let mut c = get_configuration().expect("Failed to get Configuration");
            c.database.name = Uuid::new_v4().to_string();
            c.app.port = "0".into();
            c.email_client.base_url = email_server.uri();
            c
        };

        let db_connection = configure_database(&configuration.database).await;

        // Spawn app
        let app = AppInfo::new(configuration, db_connection.clone()).expect("Failed to build app");
        let _ = tokio::spawn(app.server);
        tracing::info!("App Address: {}", app.app_address);

        TestApp {
            app_address: app.app_address,
            app_port: app.app_port,
            db_pool: db_connection,
            email_server,
        }
    }
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!(
                "{}:{}/subscriptions",
                self.app_address, self.app_port
            ))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Sending request failed!")
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
