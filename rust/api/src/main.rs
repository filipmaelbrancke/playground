use api::configuration::get_configuration;
use env_logger::Env;
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

use api::startup::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // default log level = info (used if RUST_LOG environment variable is not set)
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read the configuration.yaml file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
