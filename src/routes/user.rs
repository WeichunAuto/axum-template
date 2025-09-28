use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user;

/// Define user-related routes for the application.
///
/// - `/create_user` (POST): Calls `user::create_user` to handle user creation.
/// - `/query_user/{id}/{name}` (GET): Calls `user::get_user` to query a user.
/// - `/query_user_error/{id}/{name}` (GET): Calls `user::get_error` to demonstrate or handle an error case.
/// - Both `id` and `name` are ** path parameters ** (e.g., `/api/query_user/123/alice`).
pub(crate) fn routes() -> Router {
    Router::new()
        .route("/create_user", post(user::create_user))
        .route("/query_user/{id}/{name}", get(user::get_user))
        .route("/query_user_error/{id}/{name}", get(user::get_error))
}
