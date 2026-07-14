use alife::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use alife::viewer_server::state::new_app_state;
use std::path::PathBuf;

#[test]
fn new_app_state_starts_ready_and_idle() {
    let state = new_app_state(PathBuf::from("config/scenarios"), 300);
    let locked = state.lock().unwrap();

    assert_eq!(locked.process_state, RunnerProcessState::Ready);
    assert_eq!(locked.active_run_state, ActiveRunState::Idle);
    assert_eq!(locked.committed_tick, 0);
    assert!(locked.engine.is_none());
    assert!(locked.run_id.is_none());
    assert!(locked.scenario_hash.is_none());
}
