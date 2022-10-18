mod setup;
use setup::spawn_app;

#[tokio::test]
async fn form_post_request_operates_correctly() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=Test%20User&email=test@example.com";
    let response = client
        .post(format!("{}/subscriptions", app.app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Sending request failed!");
    let record = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to query database");

    //Assert
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(record.email, "test@example.com");
    assert_eq!(record.name, "Test User");
}

#[tokio::test]
async fn form_post_fails_correctly_with_missing_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let bad_requests = vec![
        ("email=test@example.com", "Missing Name"),
        ("name=Test%20User", "Missing Email"),
        ("", "Blank Request"),
    ];

    for (body, error_message) in bad_requests {
        //Act
        let response = client
            .post(format!("{}/subscriptions", app.app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Sending request failed!");

        //Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "API Should have failed with 400 on test: {}",
            error_message
        );
    }
}
