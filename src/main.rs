use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::config;
use zero2prod::startup::run;
use zero2prod::observability::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = config::get_config().expect("failed to read config");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to get postgres conn");

    run(listener, connection_pool)?.await
}
