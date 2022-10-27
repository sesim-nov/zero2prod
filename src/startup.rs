use crate::configuration::Settings;
use crate::mail::EmailClient;
use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Structure used to contain information about a running z2p app server.
pub struct AppInfo {
    /// Server object used to run the server.
    pub server: Server,
    /// Connection address
    pub app_address: String,
}

impl AppInfo {
    pub fn new(configuration: Settings, db_connection: PgPool) -> std::io::Result<AppInfo> {
        // TCP Listener setup for App
        let address = format!("{}:{}", configuration.app.host, configuration.app.port);
        let listener = TcpListener::bind(address)?;
        let address = format!(
            "http://{}:{}",
            configuration.app.host,
            listener.local_addr().unwrap().port()
        );

        // Email Client Setup
        let sender = configuration
            .email_client
            .sender()
            .expect("Failed to parse sender email");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            sender,
            configuration.email_client.base_url,
            configuration.email_client.auth_token,
            timeout,
        );

        // FIRE!
        match run(listener, db_connection, email_client) {
            Ok(srv) => Ok(AppInfo {
                server: srv,
                app_address: address,
            }),
            Err(e) => Err(e),
        }
    }
}

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
