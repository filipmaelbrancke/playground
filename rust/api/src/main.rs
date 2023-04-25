use std::io::Error;
use std::net::TcpListener;

use api::configuration::get_configuration;
use api::email_client::EmailClient;
use api::startup::run;
use api::telemetry;
use sqlx::postgres::PgPoolOptions;
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
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await
}
