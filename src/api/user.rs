use crate::application::AppState;
use crate::handlers::user;
use axum::routing::{delete, patch};
use axum::{
    routing::{get, post},
    Router,
};

/// Define user-related api for the application.
///
/// - `/create_user` (POST): Calls `user::create_user` to handle user creation.
/// - `/query_user/{id}/{name}` (GET): Calls `user::get_user` to query a user.
/// - `/query_user_error/{id}/{name}` (GET): Calls `user::get_error` to demonstrate or handle an error case.
/// - Both `id` and `name` are ** path parameters ** (e.g., `/api/query_user/123/alice`).
// #[axum::debug_handler]
pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/create_user", post(user::create))
        .route("/get_user", get(user::query))
        .route(
            "/update_user_ws_by_id/{id}/{ws_id}",
            patch(user::update_ws_by_id),
        )
        .route("/delete_user_by_id/{id}", delete(user::delete_by_id))
}
