use tracing::subscriber::{set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(name: String, filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    subscriber
}
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Make sure logger logs wind up in the tracer output.
    tracing_log::LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber!");
}
