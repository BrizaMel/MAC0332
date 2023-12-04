use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}
