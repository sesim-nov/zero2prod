use super::token;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct Token {
    pub token: String,
}

/// Confirm that a user email address is controlled by the initial requestor.
///
/// This endpoint uses the user's subscription token to validate that the user actually controls
/// the registered e-mail. It handles database management as well as user feedback for the
/// confirmation.
#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "Subscriber Confirmation endpoint", skip(query))]
pub async fn handle_confirm(
    query: web::Query<Token>,
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let id = match token::get_id_for_token(query.token.clone(), &pool).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError(),
    };

    let id = match id {
        Some(id) => id,
        None => {
            tracing::error!("No such token {} found", query.token);
            return HttpResponse::Unauthorized();
        }
    };

    match confirm_id(id, &pool).await {
        Ok(_) => {
            tracing::info!("User confirmation successful!");
            HttpResponse::Ok()
        }
        Err(_) => {
            tracing::error!("User confirmation query failed!");
            HttpResponse::InternalServerError()
        }
    }
}

async fn confirm_id(id: uuid::Uuid, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE subscriptions SET status = 'confirmed' WHERE id = $1
        "#,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
