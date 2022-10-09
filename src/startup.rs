use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection: PgPool) -> std::io::Result<Server> {
    let db_connection = web::Data::new(db_connection);
    let srv = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(handle_subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(srv)
}
