use actix_web::{HttpRequest, HttpResponse};

pub async fn health(_req: HttpRequest) -> HttpResponse {
    println!("server is healthy!");
    HttpResponse::Ok().finish()
}
