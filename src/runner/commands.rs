use crate::runner::lifecycle::{ActiveRunState, StateConflict};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunnerCommandKind {
    ValidateScenario,
    PrepareScenario,
    StartRun,
    PauseRun,
    ResumeRun,
    StepRun,
    StopRun,
    GetRunStatus,
}

impl RunnerCommandKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidateScenario => "ValidateScenario",
            Self::PrepareScenario => "PrepareScenario",
            Self::StartRun => "StartRun",
            Self::PauseRun => "PauseRun",
            Self::ResumeRun => "ResumeRun",
            Self::StepRun => "StepRun",
            Self::StopRun => "StopRun",
            Self::GetRunStatus => "GetRunStatus",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunnerCommand {
    ValidateScenario,
    PrepareScenario,
    StartRun,
    PauseRun,
    ResumeRun,
    StepRun,
    StopRun,
    GetRunStatus,
}

impl RunnerCommand {
    pub const fn kind(self) -> RunnerCommandKind {
        match self {
            Self::ValidateScenario => RunnerCommandKind::ValidateScenario,
            Self::PrepareScenario => RunnerCommandKind::PrepareScenario,
            Self::StartRun => RunnerCommandKind::StartRun,
            Self::PauseRun => RunnerCommandKind::PauseRun,
            Self::ResumeRun => RunnerCommandKind::ResumeRun,
            Self::StepRun => RunnerCommandKind::StepRun,
            Self::StopRun => RunnerCommandKind::StopRun,
            Self::GetRunStatus => RunnerCommandKind::GetRunStatus,
        }
    }

    pub const fn tick_budget(self) -> Option<u32> {
        match self {
            Self::StepRun => Some(1),
            _ => None,
        }
    }

    pub fn validate(self, state: ActiveRunState) -> Result<(), StateConflict> {
        match self {
            Self::ValidateScenario | Self::PrepareScenario | Self::GetRunStatus => Ok(()),
            Self::StartRun => match state {
                ActiveRunState::Idle => Ok(()),
                _ => Err(StateConflict),
            },
            Self::PauseRun => match state {
                ActiveRunState::Running => Ok(()),
                _ => Err(StateConflict),
            },
            Self::ResumeRun => match state {
                ActiveRunState::Paused => Ok(()),
                _ => Err(StateConflict),
            },
            Self::StepRun => state.validate_step_run(),
            Self::StopRun => match state {
                ActiveRunState::Preparing | ActiveRunState::Running | ActiveRunState::Paused => {
                    Ok(())
                }
                _ => Err(StateConflict),
            },
        }
    }
}
