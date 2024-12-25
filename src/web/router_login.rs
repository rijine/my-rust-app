use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{Error, Result};

pub fn router_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("{:?} - api_login HANDLER", payload);
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail)
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}