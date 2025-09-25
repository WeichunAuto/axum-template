use axum::{routing::get, Router};

use crate::handlers::user;

pub fn query_routes() -> Router {
    // 访问路径： /query_user/44/bobby
    Router::new().route("/{id}/{name}", get(user::get_user))
}

pub fn error_routes() -> Router {
    Router::new().route("/{id}/{name}", get(user::get_error))
}
