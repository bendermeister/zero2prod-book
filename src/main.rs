use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod_book::configuration::get_configuration;
use zero2prod_book::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Database");
    startup::run(listener, db_pool)?.await
}
