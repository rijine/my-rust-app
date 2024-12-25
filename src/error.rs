use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{} INTO RESPONSE", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled Internal Server Error").into_response()
    }
}

// impl std::error::Error for Error {
//     fn description(&self) -> &str {
//
//     }
//
// }