use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use once_cell::sync::Lazy;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static SUBSCRIBER: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("Tests".into(), "debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp {
    app_address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&SUBSCRIBER);
    let mut configuration = get_configuration().expect("Failed to get Configuration");
    configuration.database.db_name = Uuid::new_v4().to_string();
    let db_connection = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test port");
    let port = listener.local_addr().unwrap().port();
    let app = run(listener, db_connection.clone()).expect("Failed to spawn server");
    let _ = tokio::spawn(app);
    TestApp {
        app_address: format!("http://127.0.0.1:{}", port),
        db_pool: db_connection,
    }
}

async fn configure_database(database: &DatabaseSettings) -> PgPool {
    // Acquire a temporary connection to the main database
    let mut tmp_connection = PgConnection::connect(&database.get_connection_string_no_db())
        .await
        .expect("Temporary connection to main pg database failed.");
    tmp_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, database.db_name).as_str())
        .await
        .expect("Failed to create database");

    let conn_string = database.get_connection_string();
    let pool_out = PgPool::connect(&conn_string)
        .await
        .expect("Database connection failed.");

    sqlx::migrate!("./migrations")
        .run(&pool_out)
        .await
        .expect("Database migration failed.");

    pool_out
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Spawn the web server.
    let app = spawn_app().await;
    // reqwest is needed to comunicate with the server we spawned.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", app.app_address))
        .send()
        .await
        .expect("Request failed to execute.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn form_post_request_operates_correctly() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=Test%20User&email=test@example.com";
    let response = client
        .post(format!("{}/subscriptions", app.app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Sending request failed!");
    let record = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to query database");

    //Assert
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(record.email, "test@example.com");
    assert_eq!(record.name, "Test User");
}

#[tokio::test]
async fn form_post_fails_correctly_with_missing_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let bad_requests = vec![
        ("email=test@example.com", "Missing Name"),
        ("name=Test%20User", "Missing Email"),
        ("", "Blank Request"),
    ];

    for (body, error_message) in bad_requests {
        //Act
        let response = client
            .post(format!("{}/subscriptions", app.app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Sending request failed!");

        //Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "API Should have failed with 400 on test: {}",
            error_message
        );
    }
}
