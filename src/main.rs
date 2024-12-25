use std::net::SocketAddr;
use axum::response::Html;
use axum::{Router, ServiceExt};
use axum::routing::get;
use crate::examples::{enums, structs};

mod examples;


#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/hello", get(|| async { Html("Hello, world!") }));


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, routes.into_make_service()).await.unwrap();

    // Tryouts
    // structs();
    // enums();
}
