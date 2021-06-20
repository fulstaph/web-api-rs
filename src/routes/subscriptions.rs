use actix_web::{web, HttpResponse};
use sqlx::{PgPool};
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
    let request_id = Uuid::new_v4();
    tracing::info!("request_id {} - saving new subscriber '{}':'{}' details in the database",
        request_id,
        form.name,
        form.email
    );
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
            tracing::error!("request_id {} - failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        })?;

    tracing::info!("request_id {} - new subscriber details have been saved", request_id);

    Ok(HttpResponse::Ok().finish())
}
