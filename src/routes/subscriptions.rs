use crate::domain::ListSubscriber;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, db_connection),
    fields(
        name = %form.name,
        email = %form.email
    )
)]
pub async fn handle_subscribe(
    form: web::Form<FormData>,
    db_connection: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let user = match ListSubscriber::try_new(form.name.clone(), form.email.clone()) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to parse new subscriber details: {:?}", e);
            return HttpResponse::InternalServerError();
        }
    };

    match db_insert_user(user, &db_connection).await {
        Ok(_) => {
            tracing::info!("Database modification successful!");
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
        }
    }
}

#[tracing::instrument(name = "Adding user to database", skip(subscriber, db_connection))]
async fn db_insert_user(
    subscriber: ListSubscriber,
    db_connection: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    // Query!
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        //form.email,
        //form.name,
        subscriber.email,
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_connection)
    .await?;
    Ok(())
}
