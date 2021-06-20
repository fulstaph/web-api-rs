use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Resource};
use sqlx::{PgConnection, PgPool};
use std::sync::Arc;
use uuid::Uuid;
use std::ops::Deref;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // retrieving a connection from the application state
    connection: web::Data<Arc<PgPool>>
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(connection.get_ref().deref())
        .await
        .map_err(|e| {
            eprintln!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}
