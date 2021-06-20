use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use sqlx::{PgConnection, PgPool};

use crate::routes::{health, subscribe};
use std::sync::Arc;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Arc::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
            .data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
