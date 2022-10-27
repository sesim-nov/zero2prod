use crate::setup::TestApp;

#[tokio::test]
async fn subscribe_fails_on_empty_required_fields() {
    // Arrange
    let bad_data = vec![
        ("name=&email=test1@example.com", "empty name"),
        ("name=Tommy&email=", "empty email"),
        ("name=&email=", "all empty"),
    ];

    let app = TestApp::spawn_new().await;

    for (payload, description) in bad_data {
        // Act
        let response = app.post_subscriptions(payload.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The app did not return 400 when the request was {}",
            description
        )
    }
}
