use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, ServiceExt};
use axum::extract::{Path, Query};
use axum::routing::get;
use serde::Deserialize;
use crate::examples::{enums, structs};

mod examples;


#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello_path/:name", get(handler_hello_with_path));


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, routes.into_make_service()).await.unwrap();

    // Tryouts
    // structs();
    // enums();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> GET /hello {:<12} - hello handler {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello, {name}!"))
}

async fn handler_hello_with_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> GET /hello {:<12} - hello handler {name:?}", "HANDLER");
    // let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello, {name}!"))
}