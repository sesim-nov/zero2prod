use crate::setup::TestApp;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Spawn the web server.
    let app = TestApp::spawn_new().await;
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
