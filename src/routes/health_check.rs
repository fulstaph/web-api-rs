use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

pub async fn health(_req: HttpRequest) -> HttpResponse {
    println!("server is healthy!");
    HttpResponse::Ok().finish()
}
