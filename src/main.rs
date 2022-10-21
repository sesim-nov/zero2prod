use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::mail::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Tracer Setup
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get configuration");
    let db_connection = PgPool::connect_lazy_with(configuration.database.with_db());
    let address = format!("{}:{}", configuration.app.host, configuration.app.port);
    let listener = TcpListener::bind(address)?;
    let sender = configuration
        .email_client
        .sender()
        .expect("Failed to parse sender email");
    let email_client = EmailClient::new(sender, configuration.email_client.base_url);
    run(listener, db_connection, email_client)?.await
}
