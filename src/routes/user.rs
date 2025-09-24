use axum::{routing::get, Router};

use crate::handlers::user;

pub fn routes() -> Router {
    // 访问路径： /query_user/44/bobby
    Router::new().route("/{id}/{name}", get(user::get_user))
}
