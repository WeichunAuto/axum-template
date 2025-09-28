use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::models::user::{User, UserResponse};

#[derive(Deserialize)]
pub struct UserQuery {
    pub active: Option<bool>,
}

/// Handler for creating a new user.
///
/// - Expects a JSON body matching the `User` struct.
/// - Returns a `UserResponse` wrapped in JSON.
pub async fn create_user(Json(user): Json<User>) -> Json<UserResponse> {
    let new_user = UserResponse {
        id: 1,
        name: user.name,
        active: true,
    };
    Json(new_user)
}

/// Handler for fetching a user by `id` and `name`.
///
/// Example request: `http://127.0.0.1:8099/api/query_user/44/bobby?active=true`
pub async fn get_user(
    Path((id, name)): Path<(u32, String)>,
    Query(params): Query<UserQuery>,
) -> Json<UserResponse> {
    let user = UserResponse {
        id,
        name,
        active: params.active.unwrap_or(false), // If not provided, defaults to `false`.
    };
    Json(user)
}

/// Handler for demonstrating error responses.
pub async fn get_error(
    Path((id, name)): Path<(u32, String)>,
) -> Result<Json<UserResponse>, StatusCode> {
    if id < 10 {
        Err(StatusCode::BAD_REQUEST)
    } else if id < 20 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json(UserResponse {
            id,
            name,
            active: false,
        }))
    }
}
