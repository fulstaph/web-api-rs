use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| 
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
        )
        .listen(listener)?
        .run();

    Ok(server)
}
