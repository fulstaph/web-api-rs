use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::config::get_config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("failed to read config");

    println!("app port: {}", configuration.application_port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    run(listener)?.await
}
