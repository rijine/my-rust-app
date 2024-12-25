use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result};
use crate::web::AUTH_TOKEN;

pub fn router_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("{:?} - api_login HANDLER", payload);
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail)
    }

    // Not real world example
    cookies.add(Cookie::new(AUTH_TOKEN, format!("{}-1.expiry.signature", payload.username)));

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