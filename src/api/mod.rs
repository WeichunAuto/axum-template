use crate::application::AppState;
use crate::error::ApiError;
use crate::handlers;
use axum::{routing::get, Router};

pub(crate) mod user;
mod workspace;

/// Create and configure application api.
///
/// - Registers `/` as the root route.
/// - Uses `.nest("/api", user::api())` to mount all api from the `user` module
///   under the `/api` path.
///   For example, if `user::api()` defines `/login`, the full path will be `/api/login`.
///   Sets a `fallback` handler for unmatched api.
pub async fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::index))
        .nest("/api", user::routes())
        .nest("/api", workspace::routes())
        .fallback(handlers::fallback)
        .method_not_allowed_fallback(async || -> ApiError {
            tracing::warn!("Method not allowed!");
            ApiError::MethodNotAllowedError
        })
}
