use tokio::net::TcpListener;
use axum::{
    Router,
    routing::get,
    serve::serve
};

async fn root() -> &'static str {
    println!("Endpoint hit: /");
    "Welcome to the Rust API"
}

#[tokio::main]
async fn main() {

   let axum_router = Router::new().route("/", get(root));

    let network_address: &str = "127.0.0.1:7878";

    let tcp_listener = TcpListener::bind(network_address)
        .await
        .expect("TCP listener failed to connect");

    println!("Listening on:");
    println!("http://{network_address}");

    println!();

    serve(tcp_listener, axum_router)
        .await
        .expect("Axum failed failed to connect");
}
