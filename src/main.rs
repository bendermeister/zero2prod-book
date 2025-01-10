use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

use zero2prod_book::configuration::get_configuration;
use zero2prod_book::startup;
use zero2prod_book::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, db_pool)?.await?;
    Ok(())
}
