use alife::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use alife::viewer_server::state::{new_app_state, request_graceful_shutdown};
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(PathBuf::from("config/scenarios"), 20, 30)
}

#[test]
fn graceful_shutdown_moves_process_to_shutting_down() {
    let state = make_state();

    request_graceful_shutdown(&state);

    let locked = state.lock().unwrap();
    assert_eq!(locked.process_state, RunnerProcessState::ShuttingDown);
    assert_eq!(locked.active_run_state, ActiveRunState::Idle);
}

#[test]
fn graceful_shutdown_rejects_future_starts_without_fabricating_failed_run() {
    let state = make_state();

    request_graceful_shutdown(&state);

    let locked = state.lock().unwrap();
    assert_eq!(locked.process_state, RunnerProcessState::ShuttingDown);
    assert_eq!(locked.active_run_state, ActiveRunState::Idle);
    assert!(locked.engine.is_none());
    assert!(locked.tick_signal.is_none());
}
