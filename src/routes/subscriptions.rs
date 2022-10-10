use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn handle_subscribe(
    form: web::Form<FormData>,
    db_connection: web::Data<sqlx::PgPool>,
) -> impl Responder {
    // Error Handling stuff.
    let request_id = Uuid::new_v4();
    let span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        %form.name,
        %form.email
    );
    let _guard = span.enter();

    // Query!
    let query_span = tracing::info_span!("Saving details to the database.");
    let status = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_connection.get_ref())
    .instrument(query_span)
    .await;
    match status {
        Ok(_) => {
            tracing::info!("{} || Database modification successful!", request_id);
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("{} || Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError()
        }
    }
}
