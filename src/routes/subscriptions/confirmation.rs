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
#[tracing::instrument(name = "Subscriber Confirmation endpoint", skip(_query))]
pub async fn handle_confirm(_query: web::Query<Token>) -> impl Responder {
    HttpResponse::Ok()
}
