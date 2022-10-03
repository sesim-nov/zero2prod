use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Spawn the web server.
    let app_addr = spawn_app();
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test port");
    let port = listener.local_addr().unwrap().port();
    let app = run(listener).expect("Failed to spawn server");
    let _ = tokio::spawn(app);
    format!("http://127.0.0.1:{}", port)
}
