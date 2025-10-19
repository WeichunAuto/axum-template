use axum_template::{config, logger, routes};

#[tokio::main]
async fn main() {
    logger::init();

    let app_config = config::AppConfig::get_config();

    let app = routes::create_routes();
    let addr = format!(
        "{}:{}",
        app_config.server.get_host(),
        app_config.server.get_port()
    );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
