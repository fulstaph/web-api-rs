FROM rust

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app
COPY . .

ENTRYPOINT []
