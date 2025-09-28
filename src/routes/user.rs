use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user;

pub fn routes() -> Router {
    Router::new()
        .route("/create_user", post(user::create_user))
        .route("/query_user/{id}/{name}", get(user::get_user))
        .route("/query_user_error/{id}/{name}", get(user::get_error))
}
