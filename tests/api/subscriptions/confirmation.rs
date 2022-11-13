use crate::setup::TestApp;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
pub async fn clicking_email_link_confirms_subscriber() {
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

    let _response = app.post_subscriptions(payload.clone()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let link = app.get_links(email_request).html;

    // Act
    reqwest::get(link)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to run query");
    assert_eq!(saved.email, user_email);
    assert_eq!(saved.name, user_name);
    assert_eq!(saved.status, "confirmed");
}

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
    let _response = app.post_subscriptions(payload).await;

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
