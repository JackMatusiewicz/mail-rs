use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/echo/{name}")]
async fn index(req: HttpRequest) -> impl Responder {
    let response = format!("Hello, {}", req.match_info().get("name").unwrap_or("anon"));
    HttpResponse::Ok().body(response)
}

async fn manual_greet() -> impl Responder {
    HttpResponse::Ok().body("manual response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/manual", web::get().to(manual_greet))
    })
    .backlog(1000)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
