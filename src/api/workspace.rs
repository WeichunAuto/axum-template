use crate::application::AppState;
use crate::handlers::workspace;
use axum::routing::post;
use axum::Router;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/create_workspace", post(workspace::create_workspace))
}
