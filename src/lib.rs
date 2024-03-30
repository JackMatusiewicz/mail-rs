use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/echo/{name}")]
pub async fn index(req: HttpRequest) -> impl Responder {
    let response = format!("Hello, {}", req.match_info().get("name").unwrap_or("anon"));
    HttpResponse::Ok().body(response)
}

pub async fn manual_greet() -> impl Responder {
    HttpResponse::Ok().body("manual response")
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run_server<A: std::net::ToSocketAddrs>(address: A) -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .route("/manual", web::get().to(manual_greet))
    })
    .backlog(1000)
    .bind(address)?
    .run()
    .await
}
