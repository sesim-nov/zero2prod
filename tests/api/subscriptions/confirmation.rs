use crate::setup::TestApp;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
pub async fn confirm_returns_200_with_valid_data() {
    // Arrange
    let app = TestApp::spawn_new().await;
    Mock::given(path("/subscription/confirm"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    //TODO: Token format
    let token = "TOKEN";

    // Act
    let response = reqwest::get(format!(
        "{}/subscriptions/confirm?token={}",
        app.app_address, token
    ))
    .await
    .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
pub async fn confirm_returns_400_with_no_token() {
    // Arrange
    let app = TestApp::spawn_new().await;
    Mock::given(path("/subscription/confirm"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = reqwest::get(format!("{}/subscriptions/confirm", app.app_address))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}
