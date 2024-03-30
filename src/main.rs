use mail_rs::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (_, server) = run_server(("127.0.0.1", 8080))?;
    run_server()?.await
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use actix_web::{http::StatusCode, App};
    use mail_rs::*;

    #[actix_web::test]
    async fn health_check_expected_result() {
        let test_app = test::init_service(App::new().service(health_check)).await;
        let request = test::TestRequest::get().uri("/health").to_request();
        let response = test::call_service(&test_app, request).await;

        assert_eq!(StatusCode::OK, response.status())
    }
}
