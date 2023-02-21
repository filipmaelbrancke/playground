use std::io::Error;

#[tokio::test]
async fn health_check_should_return_ok() {
    // Arrange
    spawn_api_app().await.expect("Failed to spawn the api app");
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

async fn spawn_api_app() -> Result<(), Error> {
    todo!()
}
