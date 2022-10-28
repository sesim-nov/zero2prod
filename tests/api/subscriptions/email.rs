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
