use api::configuration::{get_configuration, DatabaseSettings};
use api::startup::{get_connection_pool, Application};
use api::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

pub static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // cannot assign output of `get_subscriber` to a variable based on value TEST_LOG
    // since the sink is part of the type returned by `get_subscriber` (and therefore
    // not the same type).
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_api_app() -> TestApp {
    Lazy::force(&TRACING); // execute code in `TRACING` on first invocation, skip on other invocations

    // mock server as stand in for the Postmark API
    let email_server = MockServer::start().await;

    // randomise configuration for test isolation
    let configuration = {
        let mut config = get_configuration().expect("Failed to read configuration");
        config.database.database_name = Uuid::new_v4().to_string(); // different database for each test
        config.application.port = 0; // random OS port
        config.email_client.base_url = email_server.uri();
        config
    };

    // create + migrate database
    configure_database(&configuration.database).await;

    // launch application as background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    // get port before spawning application
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
