use std::env;

use axum_template::routes;

#[tokio::main]
async fn main() {
    // Read environment variable APP_HOST and APP_PORT. If not set, fallback to "127.0.0.1 and 3000".
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());

    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
