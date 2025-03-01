use std::net::{Ipv4Addr, SocketAddr};
use axum::{routing::get, Router};
use tokio::net::TcpListener;

// handler
async fn lapis() -> &'static str {
    "hiro lapis"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/lapis", get(lapis));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
