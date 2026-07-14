#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunnerProcessState {
    Starting,
    Ready,
    ShuttingDown,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveRunState {
    Idle,
    Preparing,
    Running,
    Paused,
    Stopping,
    Completed,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateConflict;

impl RunnerProcessState {
    pub fn transition_to_ready(self) -> Result<Self, StateConflict> {
        match self {
            Self::Starting => Ok(Self::Ready),
            _ => Err(StateConflict),
        }
    }

    pub fn transition_to_shutting_down(self) -> Result<Self, StateConflict> {
        match self {
            Self::Ready => Ok(Self::ShuttingDown),
            _ => Err(StateConflict),
        }
    }
}

impl ActiveRunState {
    pub fn start_preparing(self) -> Result<Self, StateConflict> {
        match self {
            Self::Idle => Ok(Self::Preparing),
            _ => Err(StateConflict),
        }
    }

    pub fn finish_preparing(self) -> Result<Self, StateConflict> {
        match self {
            Self::Preparing => Ok(Self::Running),
            _ => Err(StateConflict),
        }
    }

    pub fn fail_preparing(self) -> Result<Self, StateConflict> {
        match self {
            Self::Preparing => Ok(Self::Failed),
            _ => Err(StateConflict),
        }
    }

    pub fn pause(self) -> Result<Self, StateConflict> {
        match self {
            Self::Running => Ok(Self::Paused),
            _ => Err(StateConflict),
        }
    }

    pub fn resume(self) -> Result<Self, StateConflict> {
        match self {
            Self::Paused => Ok(Self::Running),
            _ => Err(StateConflict),
        }
    }

    pub fn stop(self) -> Result<Self, StateConflict> {
        match self {
            Self::Preparing | Self::Running | Self::Paused => Ok(Self::Stopping),
            _ => Err(StateConflict),
        }
    }

    pub fn complete_stop(self) -> Result<Self, StateConflict> {
        match self {
            Self::Stopping => Ok(Self::Completed),
            _ => Err(StateConflict),
        }
    }

    pub fn reset(self) -> Result<Self, StateConflict> {
        match self {
            Self::Completed | Self::Failed => Ok(Self::Idle),
            _ => Err(StateConflict),
        }
    }

    pub fn validate_step_run(self) -> Result<(), StateConflict> {
        match self {
            Self::Paused => Ok(()),
            _ => Err(StateConflict),
        }
    }
}
