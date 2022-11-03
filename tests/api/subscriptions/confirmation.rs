use crate::setup::TestApp;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
pub async fn extracted_link_from_confirm_email_returns_200() {
    // Arrange
    let app = TestApp::spawn_new().await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let user_name: String = Name().fake();
    let user_email: String = SafeEmail().fake();
    let payload = format!("name={}&email={}", user_name, user_email);

    // Act
    let _response = app.post_subscriptions(payload.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let link = app.get_links(email_request).html;

    // Assert
    assert!(
        link.contains(&app.app_address),
        "{} doesn't contain {}",
        link,
        app.app_address
    );
    let email_response = reqwest::get(link.clone()).await.unwrap();

    assert_eq!(200, email_response.status(), "{}", link);
}

#[tokio::test]
pub async fn confirm_returns_200_with_valid_data() {
    // Arrange
    let app = TestApp::spawn_new().await;
    //TODO: Token format
    let token = "TOKEN";

    // Act
    let response = reqwest::get(format!(
        "{}:{}/subscriptions/confirm?token={}",
        app.app_address, app.app_port, token
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

    // Act
    let response = reqwest::get(format!(
        "{}:{}/subscriptions/confirm",
        app.app_address, app.app_port
    ))
    .await
    .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}
