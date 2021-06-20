use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use std::ops::Deref;
use chrono::Utc;
use tracing_futures::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // retrieving a connection from the application state
    pool: web::Data<Arc<PgPool>>
) -> Result<HttpResponse, HttpResponse> {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
      "adding new subscriber",
        %request_id,
        email = %form.email,
        name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
      "saving new subscriber details in the database"
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
        .execute(pool.as_ref().deref())
        .instrument(query_span)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    tracing::info!("request_id {} - new subscriber details have been saved", request_id);

    Ok(HttpResponse::Ok().finish())
}
