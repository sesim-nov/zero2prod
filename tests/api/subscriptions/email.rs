use crate::setup::TestApp;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
pub async fn subscribe_sends_email() {
    // Arrange
    let app = TestApp::spawn_new().await;
    let name = "Fake Name";
    let email = SafeEmail().fake::<String>();
    let body = format!("name={}&email={}", name, email);

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body).await;

    // Assert
}

#[tokio::test]
pub async fn confirmation_email_has_link() {
    // Arrange
    let app = TestApp::spawn_new().await;
    let name = "Fake Name";
    let email = SafeEmail().fake::<String>();
    let body = format!("name={}&email={}", name, email);

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body).await;

    // Assert
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let links = app.get_links(email_request);
    assert_eq!(links.html, links.plain_text);
}

#[tokio::test]
pub async fn subscribing_twice_sends_dual_emails() {
    // Arrange
    let app = TestApp::spawn_new().await;
    let name = "Fake Name";
    let email = SafeEmail().fake::<String>();
    let body = format!("name={}&email={}", name, email);

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(2)
        .mount(&app.email_server)
        .await;

    // Act
    for _ in 0..2 {
        let response = app.post_subscriptions(body.clone()).await;
        // Assert
        assert_eq!(response.status(), 200);
    }
}
