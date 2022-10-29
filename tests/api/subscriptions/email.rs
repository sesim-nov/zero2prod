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
    let get_link = |s: &str| -> String {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };
    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let email_body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let html_link = get_link(email_body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(email_body["TextBody"].as_str().unwrap());

    assert_eq!(html_link, text_link);
}
