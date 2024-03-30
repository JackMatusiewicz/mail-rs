use actix_web::test;
use actix_web::{http::StatusCode, App};
use mail_rs::*;

#[actix_web::test]
pub async fn server_test() {
    run_server(("127.0.0.1", 8080)).await.expect("Server failed to start");

    let http_client = reqwest::Client::new();

    let response = http_client
        .get("http://127.0.0.1:8000/health")
        .send()
        .await
        .expect("Expected a response");

    assert!(response.status().is_success());
}
