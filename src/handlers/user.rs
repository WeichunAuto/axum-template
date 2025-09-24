use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;

use crate::models::user::UserResponse;

#[derive(Deserialize)]
pub struct UserQuery {
    pub active: Option<bool>,
}

// 从 URL Path 中接收字段 id, name
// 访问路径：http://127.0.0.1:8099/users/44/bobby?active=true
pub async fn get_user(
    Path((id, name)): Path<(u32, String)>,
    Query(query): Query<UserQuery>,
) -> Json<UserResponse> {
    let user = UserResponse {
        id,
        name,
        active: query.active.unwrap_or(false), // 如果未带 active 查询参数，则默认false
    };
    Json(user)
}
