use std::collections::HashMap;

use axum::{
    Json, extract::{FromRequest, State, rejection::JsonRejection}, http::StatusCode, response::IntoResponse
};
use serde::Serialize;
use serde_json::json;

use crate::{cmd, CommandHandler, RestReq, RestRes};

#[derive(Clone)]
pub struct AppState {
    handlers: HashMap<&'static str, CommandHandler>
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
    ) -> impl IntoResponse {
        let res = if let Some(func) = state.handlers.get(value.command.as_str()) {
            func(&value).expect("JSON response")
        } else {
            // return Err(ApiError{status: StatusCode::INTERNAL_SERVER_ERROR, message: "what?".to_string()});
            panic!("だめ")
        };
        Json(res)
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
