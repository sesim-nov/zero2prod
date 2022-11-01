use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct Token {
    pub token: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "Subscriber Confirmation endpoint", skip(_query))]
pub async fn handle_confirm(_query: web::Query<Token>) -> impl Responder {
    HttpResponse::Ok()
}
