use axum::{routing::get_service, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().nest_service("/", get_service(ServeDir::new("www")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("LED Simulator (WASM) running at http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}
