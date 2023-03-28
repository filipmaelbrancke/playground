use reqwest::StatusCode;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_should_return_ok() {
    // Arrange
    let address = spawn_api_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_should_return_ok_for_valid_form_data() {
    // Arrange
    let app_address = spawn_api_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(StatusCode::OK, response.status());
}

#[tokio::test]
async fn subscribe_should_fail_with_a_400_when_missing_data() {
    // Arrange
    let app_address = spawn_api_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content_Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

fn spawn_api_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port on localhost");
    let port = listener.local_addr().unwrap().port();
    let server = api::run(listener).expect("Failed to bind");
    // launch server as background task
    // tokio::spawn returns handle to spawned future
    // not used here -> hence non-binding let
    let _ = tokio::spawn(server);
    // return address:port
    format!("http://127.0.0.1:{}", port)
}
