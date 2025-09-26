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

// 创建一个 user
pub async fn create_user(Json(user): Json<User>) -> Json<UserResponse> {
    let new_user = UserResponse {
        id: 1,
        name: user.name,
        active: true,
    };
    Json(new_user)
}

// 从 URL Path 中接收字段 id, name
// 访问路径：http://127.0.0.1:8099/query_user/44/bobby?active=true
pub async fn get_user(
    Path((id, name)): Path<(u32, String)>,
    Query(params): Query<UserQuery>,
) -> Json<UserResponse> {
    let user = UserResponse {
        id,
        name,
        active: params.active.unwrap_or(false), // 如果未带 active 查询参数，则默认false
    };
    Json(user)
}

/**
 * 返回错误码的 handler 函数
 */
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
