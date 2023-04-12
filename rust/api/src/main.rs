use std::io::Error;
use std::net::TcpListener;

use sqlx::PgPool;

use api::configuration::get_configuration;
use api::startup::run;
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

    let configuration = get_configuration().expect("Failed to read the configuration.yaml file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
