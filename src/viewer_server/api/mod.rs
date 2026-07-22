pub mod info;
pub mod projections;
pub mod run;
pub mod scenarios;
pub mod stream;

use crate::viewer_server::state::AppState;
use axum::Router;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(info::router(state.clone()))
        .merge(projections::router(state.clone()))
        .merge(scenarios::router(state.clone()))
        .merge(run::router(state.clone()))
        .merge(stream::router(state))
}
