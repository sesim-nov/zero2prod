use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to get Configuration");
    let conn_string = configuration.database.get_connection_string();
    let db_connection = PgConnection::connect(&conn_string)
        .await
        .expect("Database connection failed.");
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind test port");
    let port = listener.local_addr().unwrap().port();
    let app = run(listener, db_connection).expect("Failed to spawn server");
    let _ = tokio::spawn(app);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Spawn the web server.
    let app_addr = spawn_app().await;
    // reqwest is needed to comunicate with the server we spawned.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", app_addr))
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
    let app_addr = spawn_app().await;
    let configuration = get_configuration().expect("Failed to get Configuration");
    let conn_string = configuration.database.get_connection_string();
    let mut db_connection = PgConnection::connect(&conn_string)
        .await
        .expect("Database connection failed.");
    let client = reqwest::Client::new();

    //Act
    let body = "name=Test%20User&email=test@example.com";
    let response = client
        .post(format!("{}/subscriptions", app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Sending request failed!");
    let record = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut db_connection)
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
    let app_addr = spawn_app().await;
    let client = reqwest::Client::new();
    let bad_requests = vec![
        ("email=test@example.com", "Missing Name"),
        ("name=Test%20User", "Missing Email"),
        ("", "Blank Request"),
    ];

    for (body, error_message) in bad_requests {
        //Act
        let response = client
            .post(format!("{}/subscriptions", app_addr))
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
