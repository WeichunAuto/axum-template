use crate::response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFoundError,

    #[error("Method Not Allowed")]
    MethodNotAllowedError,

    #[error("Biz Success")]
    Biz,

    #[error("Database Error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Internal Server Error: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFoundError => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowedError => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz => StatusCode::OK,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = axum::Json(ApiResponse::<()>::error(self.to_string()));
        (status_code, body).into_response()
    }
}
