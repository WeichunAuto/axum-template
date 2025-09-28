use axum::http::StatusCode;

pub(crate) mod user;

/// Index handler
///
/// This is a simple async handler for the root route (`/`).
pub(crate) async fn index() -> &'static str {
    "index"
}

/// Fallback handler
///
/// This async function is used as the default route handler
/// when no other routes match the incoming request.
/// It returns a `404 Not Found` status with a simple error message.
pub(crate) async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Oops, this route is not exist.")
}
