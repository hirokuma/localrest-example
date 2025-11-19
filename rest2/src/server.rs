use std::collections::HashMap;

use anyhow::Result;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{CommandHandler, RestReq, RestRes, cmd};

#[derive(Clone)]
pub struct AppState {
    handlers: HashMap<&'static str, CommandHandler>,
}

impl AppState {
    pub fn new() -> Self {
        let handlers = cmd::register_handle();

        Self { handlers }
    }

    // "/" に対してここが post で呼ばれる
    pub async fn handler(
        State(state): State<Self>,
        Json(value): Json<RestReq>,
    ) -> Result<Json<RestRes>, AppError> {
        let res = if let Some(func) = state.handlers.get(value.command.as_str()) {
            func(&value)?
        } else {
            return Err(AppError(anyhow::anyhow!("だめ")));
        };
        Ok(Json(res))
    }
}

// https://github.com/tokio-rs/axum/blob/b1ef45469bf8ffa334e86ddd12e7f4d4b82fa1ab/examples/anyhow-error-response/src/main.rs
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let payload = json!({
            "message": "not success",
            "from": self.0.to_string(),
        });
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(payload),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/*
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
struct AppJson<T>(T);

// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message,
        });

        (self.status, axum::Json(payload)).into_response()
    }
}
*/
