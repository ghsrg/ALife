use alife::runner::lifecycle::{ActiveRunState, RunnerProcessState};

#[test]
fn canon_process_state_transitions_are_enforced() {
    assert_eq!(
        RunnerProcessState::Starting.transition_to_ready().unwrap(),
        RunnerProcessState::Ready
    );
    assert!(RunnerProcessState::Ready.transition_to_ready().is_err());
    assert_eq!(
        RunnerProcessState::Ready
            .transition_to_shutting_down()
            .unwrap(),
        RunnerProcessState::ShuttingDown
    );
}

#[test]
fn canon_active_run_state_transitions_are_enforced() {
    let running = ActiveRunState::Idle
        .start_preparing()
        .unwrap()
        .finish_preparing()
        .unwrap();
    assert_eq!(running, ActiveRunState::Running);
    assert_eq!(running.pause().unwrap(), ActiveRunState::Paused);
    assert!(ActiveRunState::Idle.pause().is_err());
}

#[test]
fn step_run_never_advances_from_running_state() {
    assert!(ActiveRunState::Paused.validate_step_run().is_ok());
    assert!(ActiveRunState::Running.validate_step_run().is_err());
}
