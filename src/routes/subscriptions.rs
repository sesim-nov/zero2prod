use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code, unused_variables)]
pub async fn handle_subscribe(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
