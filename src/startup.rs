use crate::mail::EmailClient;
use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_connection: PgPool,
    email_client: EmailClient,
) -> std::io::Result<Server> {
    let db_connection = web::Data::new(db_connection);
    let email_client = web::Data::new(email_client);
    let srv = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(handle_subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(db_connection.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(srv)
}
