use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/echo/{name}")]
async fn index(req: HttpRequest) -> impl Responder {
    let response = format!("Hello, {}", req.match_info().get("name").unwrap_or("anon"));
    HttpResponse::Ok().body(response)
}

async fn manual_greet() -> impl Responder {
    HttpResponse::Ok().body("manual response")
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

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
    use actix_web::test;

    #[actix_web::test]
    async fn health_check_expected_result() {
        let test_app = test::init_service(App::new().service(health_check)).await;
        let request = test::TestRequest::get().uri("/health").to_request();
        let response = test::call_service(&test_app, request).await;

        assert_eq!(StatusCode::OK, response.status())
    }
}
