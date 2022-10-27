use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::AppInfo;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Tracer Setup
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Configuration
    let configuration = get_configuration().expect("Failed to get configuration");
    // SQL Database setup
    let db_connection = PgPool::connect_lazy_with(configuration.database.with_db());

    let app = AppInfo::new(configuration, db_connection)?;
    app.server.await?;
    Ok(())
}
