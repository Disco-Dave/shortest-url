use common::spawn_app;

mod common;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(reqwest::StatusCode::NO_CONTENT, response.status());
    assert_eq!(Some(0), response.content_length());
}
