use crate::domain::{ListSubscriber, ListSubscriberEmail, ListSubscriberName};
use crate::mail::{EmailClient, EmailMessage};
use crate::startup::AppBaseUrl;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

mod confirmation;
pub use confirmation::*;

mod token;

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
    email_client: web::Data<EmailClient>,
    base_url: web::Data<AppBaseUrl>,
) -> impl Responder {
    let user: ListSubscriber = match form.0.try_into() {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to parse new subscriber details: {:?}", e);
            return HttpResponse::BadRequest();
        }
    };
    
    let subscriber_id = db_insert_user(&user, &db_connection).await;

    match subscriber_id {
        Ok(_) => {
            tracing::info!("Database modification successful!");
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            return HttpResponse::InternalServerError();
        }
    }

    match send_confirmation_email(email_client.get_ref(), user, base_url.get_ref()).await {
        Ok(_) => {
            tracing::info!("Email sent");
        }
        Err(e) => {
            tracing::error!("Failed to send email. {:?}", e);
            return HttpResponse::InternalServerError();
        }
    }

    HttpResponse::Ok()
}

/// Send a confirmation email
#[tracing::instrument(name = "Sending confirmation email")]
async fn send_confirmation_email(
    email_client: &EmailClient,
    user: ListSubscriber,
    base_url: &AppBaseUrl,
) -> Result<reqwest::Response, reqwest::Error> {
    let confirm_link = format!("{}/subscriptions/confirm?token=TODO", base_url.0);
    let message = EmailMessage {
        recipient: user.email,
        subject: "Derp".into(),
        body_text: format!("Welcome to my mailing list. Link: {}", confirm_link),
        body_html: format!("Welcome to my list <a href={}>Link</a>", confirm_link),
    };

    email_client.send_mail(message).await
}

/// Insert a user into the database
/// By default, the user is inserted as pending confirmation.
#[tracing::instrument(name = "Adding user to database", skip(subscriber, db_connection))]
async fn db_insert_user(
    subscriber: &ListSubscriber,
    db_connection: &sqlx::PgPool,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    // Query!
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending')
        "#,
        subscriber_id,
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_connection)
    .await?;
    Ok(subscriber_id)
}
