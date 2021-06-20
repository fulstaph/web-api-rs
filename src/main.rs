use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::config;
use sqlx::{PgPool};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = config::get_config().expect("failed to read config");

    println!("app port: {}", configuration.application_port);

    let listener = TcpListener::bind(
        format!("127.0.0.1:{}", configuration.application_port)
    )?;

    let connection_pool = PgPool::connect(
        &configuration.database.connection_string())
        .await
        .expect("failed to get postgres conn");

    run(listener, connection_pool)?.await
}
