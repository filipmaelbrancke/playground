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

fn spawn_api_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port on localhost");
    let port = listener.local_addr().unwrap().port();
    let server = api::run(listener).expect("Failed to bind");
    // launch server as background task
    // tokio::spawn returns handle to spawned future
    // not used here -> hence non-binding let
    let _ = tokio::spawn(server);
    // return address:port
    format!("http://127.0.0.1:{}", port)
}
