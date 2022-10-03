use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Spawn the web server.
    spawn_app();
    // reqwest is needed to comunicate with the server we spawned.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Request failed to execute.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let app = run().expect("Failed to spawn server");
    let _ = tokio::spawn(app);
}
