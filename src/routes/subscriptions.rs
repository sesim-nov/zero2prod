use crate::domain::{ListSubscriber, ListSubscriberEmail, ListSubscriberName};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for ListSubscriber {
    type Error = String;
    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = ListSubscriberName::try_from(form.name)?;
        let email = ListSubscriberEmail::try_from(form.email)?;
        Ok(Self { name, email })
    }
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
    let user = match form.0.try_into() {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to parse new subscriber details: {:?}", e);
            return HttpResponse::BadRequest();
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
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        //form.email,
        //form.name,
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_connection)
    .await?;
    Ok(())
}
