use alife::runner::lifecycle::{ActiveRunState, RunnerProcessState};

#[test]
fn process_starts_ready_and_can_shutdown() {
    let state = RunnerProcessState::Starting;

    assert_eq!(
        state.transition_to_ready().unwrap(),
        RunnerProcessState::Ready
    );
    assert_eq!(
        RunnerProcessState::Ready
            .transition_to_shutting_down()
            .unwrap(),
        RunnerProcessState::ShuttingDown
    );
}

#[test]
fn active_run_start_pause_resume_stop_flow_is_canonical() {
    let state = ActiveRunState::Idle
        .start_preparing()
        .unwrap()
        .finish_preparing()
        .unwrap()
        .pause()
        .unwrap()
        .resume()
        .unwrap()
        .stop()
        .unwrap()
        .complete_stop()
        .unwrap()
        .reset()
        .unwrap();

    assert_eq!(state, ActiveRunState::Idle);
}

#[test]
fn step_run_is_valid_only_when_paused() {
    assert!(ActiveRunState::Paused.validate_step_run().is_ok());
    assert!(ActiveRunState::Running.validate_step_run().is_err());
    assert!(ActiveRunState::Idle.validate_step_run().is_err());
}
