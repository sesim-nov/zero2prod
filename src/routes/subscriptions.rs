use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
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
    let request_id = Uuid::new_v4();
    log::info!("{} || Beginning subscription of new user {} with email {}", request_id, form.name, form.email);
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
    .await;
    match status {
        Ok(_) => {
            log::info!("{} || Database modification successful!", request_id);
            HttpResponse::Ok()
        },
        Err(e) => {
            log::error!("{} || Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError()
        }
    }
}
