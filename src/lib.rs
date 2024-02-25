use actix_web::{get, HttpRequest, HttpResponse, Responder};

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