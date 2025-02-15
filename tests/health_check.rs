#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    // We need to bring in `reqwest` to perform HTTP requests against our
    // application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = news_letter::run(listener).expect("Failed to start server");
    let _ = tokio::spawn(async move { server.await });
    // We return the application address to the caller!
    format!("http://127.0.0.1:{port}")
}
