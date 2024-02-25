use mail_rs::*;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .route("/manual", web::get().to(manual_greet))
    })
    .backlog(1000)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, App};
    use mail_rs::*;
    use actix_web::test;

    #[actix_web::test]
    async fn health_check_expected_result() {
        let test_app = test::init_service(App::new().service(health_check)).await;
        let request = test::TestRequest::get().uri("/health").to_request();
        let response = test::call_service(&test_app, request).await;

        assert_eq!(StatusCode::OK, response.status())
    }
}
