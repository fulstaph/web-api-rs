use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

pub async fn health(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
