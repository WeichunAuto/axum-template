use std::env;

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(fn_foo))
        .route("/pfoo", post(fn_pfoo))
        .route("/json", get(fn_json));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, this is a rust World watch!"
}

async fn fn_foo() -> &'static str {
    "Hey, this is get request fn_foo"
}

// post 请求
async fn fn_pfoo() -> &'static str {
    "Hey, this is post request pfoo"
}
// 返回 json
async fn fn_json() -> Json<Value> {
    Json(json!({"data": 44}))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
