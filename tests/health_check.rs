use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let address = format!("{address}/health_check");

    let response = client
        .get(&address)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("could not bind tcp listener");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod_book::run(listener).expect("Could not create server");
    tokio::spawn(server);

    format!("http://localhost:{port}")
}
