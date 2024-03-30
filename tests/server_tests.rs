use mail_rs::*;

#[actix_web::test]
pub async fn server_health_check_works() {
    let (port, server) =
        run_server(("127.0.0.1", 0))
            .expect("Server failed to start");

    let _ = tokio::spawn(server);

    let http_client = reqwest::Client::new();

    let health_check_url = format!("http://127.0.0.1:{}/health", port);

    let response = http_client
        .get(health_check_url)
        .send()
        .await
        .expect("Expected a response");

    assert!(response.status().is_success());
}
