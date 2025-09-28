use axum::Router;

pub mod user;

pub fn create_routes() -> Router {
    // 所有 query_user 相关路由放在 /query_user 下
    Router::new().nest("/api", user::routes())
}
