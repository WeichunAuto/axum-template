use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use validator::ValidationErrors;
// use axum_extra::extract::QueryRejection;

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

    #[error("Query Params Error: {0}")]
    QueryError(#[from] QueryRejection),

    #[error("Path Params Error: {0}")]
    PathError(#[from] PathRejection),

    #[error("Json Body Error: {0}")]
    JsonError(#[from] JsonRejection),

    #[error("Validation Error: {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFoundError => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowedError => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz => StatusCode::OK,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::QueryError(_)
            | ApiError::PathError(_)
            | ApiError::JsonError(_)
            | ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
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
