use std::io::Error;

use api::configuration::get_configuration;
use api::startup::Application;
use api::telemetry;
use telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber(
        // output formatted spans to stdout
        "api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
