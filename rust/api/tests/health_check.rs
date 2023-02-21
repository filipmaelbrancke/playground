use std::io::Error;

#[tokio::test]
async fn health_check_should_return_ok() {
    // Arrange
    spawn_api_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_api_app() {
    let server = api::run().expect("Failed to bind");
    // launch server as background task
    // tokio::spawn returns handle to spawned future
    // not used here -> hence non-binding let
    let _ = tokio::spawn(server);
}
