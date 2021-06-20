use std::net::TcpListener;
use zero2prod::config;
use zero2prod::startup::run;
use zero2prod::observability::*;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);

    let configuration = config::get_config().expect("failed to read config");

    let listener = TcpListener::bind(
        format!("{}:{}", configuration.application.host , configuration.application.port))?;

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(configuration.database.connection_timeout))
        .connect(&configuration.database.connection_string())
        .await
        .expect("failed to get postgres conn");

    run(listener, connection_pool)?.await
}
