use axum::Router;

pub mod user;

pub fn create_routes() -> Router {
    // 所有 query_user 相关路由放在 /query_user 下
    Router::new()
        .nest("/query_user", user::query_routes())
        .nest("/create_user", user::create_routes())
        .nest("/query_user_error", user::error_routes())
}
