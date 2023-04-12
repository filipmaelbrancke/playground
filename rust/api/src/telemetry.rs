use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

/*
Compose multiple layers into a `tracing`'s subscriber
Use `impl Subscriber` as return type to avoid having to spell out
actual type of returned subscriber
*/
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // higher-ranked trait bound (HRTB)
    // Sink implements the `MakeWriter`trait for all choices of the
    // lifetime parameter `'a`
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // default log level = info (used if RUST_LOG environment variable is not set)
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/*
Register subscriber as global default to pocess span data
*/
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // use `set_global_default` to specify which subscriber should be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}
