use axum::{
    http::{header, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::manager::ManagerError;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub projection: Vec<String>,
    pub filters: String,
}

#[derive(Debug)]
pub struct SearchResponse<T> {
    pub status_code: StatusCode,
    pub header: Vec<(HeaderName, String)>,
    pub response: T,
}

impl<T> SearchResponse<T> {
    pub fn new(status_code: StatusCode, response: T) -> Self {
        let header = vec![(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            std::env::var("FRONT_END_HOST")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .to_string(),
        )];

        Self {
            status_code,
            header,
            response,
        }
    }
}

impl<T: Serialize> IntoResponse for SearchResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status_code, Json(self.response)).into_response();

        let header_map = response.headers_mut();
        for (h, v) in self.header {
            header_map.append(h, HeaderValue::from_str(&v).unwrap());
        }

        response
    }
}

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

impl From<ManagerError> for RequestError {
    fn from(error: ManagerError) -> Self {
        let (status_code, message) = match error {
            ManagerError::ParseError(e) => (StatusCode::BAD_REQUEST, e),
            ManagerError::QueryBuildError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            ManagerError::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        Self {
            status_code,
            message,
        }
    }
}
