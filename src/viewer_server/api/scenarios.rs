use crate::viewer_server::state::AppState;
use axum::Router;

pub fn router(_state: AppState) -> Router {
    Router::new()
}
