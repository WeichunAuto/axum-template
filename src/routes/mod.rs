use crate::handlers;
use axum::{routing::get, Router};

pub(crate) mod user;

/// Create and configure application routes.
///
/// - Registers `/` as the root route.
/// - Uses `.nest("/api", user::routes())` to mount all routes from the `user` module
///   under the `/api` path.
///   For example, if `user::routes()` defines `/login`, the full path will be `/api/login`.
///   Sets a `fallback` handler for unmatched routes.
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .nest("/api", user::routes())
        .fallback(handlers::fallback)
}
