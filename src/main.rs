use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to get configuration");
    let db_connection = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Failed to connect to the database");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, db_connection)?.await
}
