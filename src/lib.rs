use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/echo/{name}")]
pub async fn index(req: HttpRequest) -> impl Responder {
    let response = format!("Hello, {}", req.match_info().get("name").unwrap_or("anon"));
    HttpResponse::Ok().body(response)
}

pub async fn manual_greet() -> impl Responder {
    HttpResponse::Ok().body("manual response")
}

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run_server<A: std::net::ToSocketAddrs>(address: A) -> Result<(u16, Server), std::io::Error> {
    let listener: TcpListener = TcpListener::bind(address)?;
    let port = listener.local_addr()?.port();
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .route("/manual", web::get().to(manual_greet))
    })
    .backlog(1000)
    .listen(listener)?
    .run();

    Ok((port, server))
}
