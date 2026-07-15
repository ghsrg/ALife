pub mod api;
pub mod broadcaster;
pub mod frame_encoder;
pub mod projection_sampler;
pub mod state;

use axum::Router;
use state::AppState;

pub fn create_app(app_state: AppState) -> Router {
    api::build_router(app_state)
}
