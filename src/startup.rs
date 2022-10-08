use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection: PgConnection) -> std::io::Result<Server> {
    let db_connection = web::Data::new(db_connection);
    let srv = HttpServer::new(move || {
        App::new()
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
