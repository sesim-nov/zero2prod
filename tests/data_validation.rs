mod setup;
use setup::spawn_app;

#[tokio::test]
#[ignore]
async fn subscribe_fails_on_empty_required_fields() {
    // Arrange
    let bad_data = vec![
        ("name=&email=test1@example.com", "empty name"),
        ("name=Tommy&email=", "empty email"),
        ("name=&email=", "all empty"),
    ];

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    for (payload, description) in bad_data {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", app.app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(payload)
            .send()
            .await
            .expect("Failed to complete request.");

        // Assert
        assert_eq!(400, response.status().as_u16(), "The app did not return 400 when the request was {}", description)
    }

}
