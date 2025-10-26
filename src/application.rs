use crate::config::AppConfig;
use crate::{database, logger};
use axum::extract::Request;
use axum::http::Response;
use axum::Router;
use sea_orm::DatabaseConnection;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::trace::{OnResponse, TraceLayer};
use tracing::Span;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}
struct Server {
    config: &'static AppConfig,
}

#[derive(Debug, Clone)]
struct LatencyOnResponse;

struct Latency(Duration);

impl AppState {
    fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting the application server......");

    let db_connection = database::init().await?;

    let app_state = AppState::new(db_connection);

    let server = Server::new(AppConfig::get());

    server.start(app_state, router).await
}

impl Server {
    fn new(config: &'static AppConfig) -> Self {
        Self { config }
    }

    async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let server_config = self.config.server();
        tracing::info!("Server config: {:?}", server_config);

        let routes = self.create_routes(state, router);

        let addr = format!("{}:{}", server_config.get_host(), server_config.get_port());

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("Listening on {}", addr);
        axum::serve(
            listener,
            routes.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn create_routes(&self, state: AppState, router: Router<AppState>) -> Router {
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request: ", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);

        Router::new().merge(router).layer(tracing).with_state(state)
    }
}

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = %response.status().as_u16(),
            "finished processing request"
        )
    }
}

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} us", self.0.as_micros())
        }
    }
}
