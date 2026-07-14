use alife::runner::commands::{RunnerCommand, RunnerCommandKind};
use alife::runner::lifecycle::ActiveRunState;

#[test]
fn command_kind_is_stable_for_shared_contract() {
    assert_eq!(RunnerCommand::PauseRun.kind(), RunnerCommandKind::PauseRun);
    assert_eq!(RunnerCommand::StepRun.kind().as_str(), "StepRun");
}

#[test]
fn step_run_contract_is_exactly_one_tick_and_paused_only() {
    assert_eq!(RunnerCommand::StepRun.tick_budget(), Some(1));
    assert!(
        RunnerCommand::StepRun
            .validate(ActiveRunState::Paused)
            .is_ok()
    );
    assert!(
        RunnerCommand::StepRun
            .validate(ActiveRunState::Running)
            .is_err()
    );
}

#[test]
fn start_run_is_valid_only_when_idle() {
    let command = RunnerCommand::StartRun;

    assert!(command.validate(ActiveRunState::Idle).is_ok());
    assert!(command.validate(ActiveRunState::Running).is_err());
}
