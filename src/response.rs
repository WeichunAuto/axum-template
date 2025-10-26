use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

/// the api response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i16,
    pub msg: String,

    #[serde(skip_serializing_if = "Option::is_none")] // 忽略序列化，如果Option is none.
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i16, msg: String, data: Option<T>) -> Self {
        ApiResponse { code, msg, data }
    }

    pub fn success<M: Into<String>>(message: M, data: Option<T>) -> Self {
        ApiResponse::new(200, message.into(), data)
    }

    pub fn error<M: Into<String>>(message: M) -> Self {
        ApiResponse::new(0, message.into(), None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
