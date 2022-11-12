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
    let _id = match token::get_id_for_token(query.token.clone(), &pool).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError(),
    };
    HttpResponse::Ok()
}
