use axum::Router;

pub mod user;

/// Create and configure application routes.
///
/// - Starts with a new empty `Router`.
/// - Uses `.nest("/api", user::routes())` to mount all routes from the `user` module
///   under the `/api` path.
///   For example, if `user::routes()` defines `/login`, the full path will be `/api/login`.
pub fn create_routes() -> Router {
    Router::new().nest("/api", user::routes())
}
