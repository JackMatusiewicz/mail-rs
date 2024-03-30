use mail_rs::*;

#[actix_web::test]
pub async fn server_health_check_works() {
    let server =
        run_server(("127.0.0.1", 8080))
            .expect("Server failed to start");

    let _ = tokio::spawn(server);

    let http_client = reqwest::Client::new();

    let response = http_client
        .get("http://127.0.0.1:8080/health")
        .send()
        .await
        .expect("Expected a response");

    assert!(response.status().is_success());
}
