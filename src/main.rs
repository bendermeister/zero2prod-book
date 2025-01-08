use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    zero2prod_book::run(
        TcpListener::bind("localhost:8080").expect("Could not bind TcpListener to port"),
    )?
    .await
}
