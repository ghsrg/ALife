use crate::bootstrap::{BootstrapError, prepare};
use crate::core::snapshot::CommittedSnapshot;
use crate::core::tick::{TickError, TickExecutor};
use crate::runner::lifecycle::{ActiveRunState, StateConflict};
use crate::runner::ring_buffer::RingBuffer;
use crate::runner::scenario_doc::{ScenarioDocument, ScenarioHash};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotCadence {
    EveryTick,
    EveryNTicks(u64),
    OnDemandOnly,
}

impl SnapshotCadence {
    fn should_cache_after_tick(self, committed_tick: u64) -> bool {
        match self {
            Self::EveryTick => true,
            Self::EveryNTicks(ticks) => {
                let ticks = ticks.max(1);
                committed_tick % ticks == 0
            }
            Self::OnDemandOnly => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunEngineConfig {
    pub snapshot_buffer_size: usize,
    pub snapshot_cadence: SnapshotCadence,
}

impl Default for RunEngineConfig {
    fn default() -> Self {
        Self {
            snapshot_buffer_size: 300,
            snapshot_cadence: SnapshotCadence::EveryTick,
        }
    }
}

impl RunEngineConfig {
    pub const fn headless_debug() -> Self {
        Self {
            snapshot_buffer_size: 4,
            snapshot_cadence: SnapshotCadence::OnDemandOnly,
        }
    }
}

#[derive(Debug)]
pub enum RunEngineError {
    Bootstrap(BootstrapError),
    Tick(TickError),
    StateConflict,
    InvalidBuffer,
    NotPrepared,
}

impl fmt::Display for RunEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bootstrap(err) => write!(f, "bootstrap failed: {err}"),
            Self::Tick(err) => write!(f, "tick failed: {err:?}"),
            Self::StateConflict => f.write_str("runner state conflict"),
            Self::InvalidBuffer => f.write_str("invalid snapshot buffer"),
            Self::NotPrepared => f.write_str("run is not prepared"),
        }
    }
}

impl std::error::Error for RunEngineError {}

impl From<StateConflict> for RunEngineError {
    fn from(_: StateConflict) -> Self {
        Self::StateConflict
    }
}

impl From<TickError> for RunEngineError {
    fn from(value: TickError) -> Self {
        Self::Tick(value)
    }
}

pub struct RunEngine {
    state: ActiveRunState,
    executor: Option<TickExecutor>,
    snapshots: RingBuffer<CommittedSnapshot>,
    scenario_hash: Option<ScenarioHash>,
    max_ticks: u64,
    config: RunEngineConfig,
    snapshot_build_count: u64,
}

impl RunEngine {
    pub fn prepare_from_document(
        document: &ScenarioDocument,
        config: RunEngineConfig,
    ) -> Result<Self, RunEngineError> {
        let prepared = prepare(document).map_err(RunEngineError::Bootstrap)?;
        let executor = TickExecutor::new(prepared.runtime_config.clone())?;
        let mut snapshots = RingBuffer::new(config.snapshot_buffer_size)
            .map_err(|_| RunEngineError::InvalidBuffer)?;
        snapshots.push(CommittedSnapshot::from_world(executor.world()));
        Ok(Self {
            state: ActiveRunState::Paused,
            executor: Some(executor),
            snapshots,
            scenario_hash: Some(document.scenario_hash),
            max_ticks: prepared.runtime_config.world.tick_count.raw(),
            config,
            snapshot_build_count: 1,
        })
    }

    pub const fn state(&self) -> ActiveRunState {
        self.state
    }

    pub fn start(&mut self) -> Result<(), RunEngineError> {
        match self.state {
            ActiveRunState::Paused => {
                self.state = ActiveRunState::Running;
                Ok(())
            }
            _ => Err(RunEngineError::StateConflict),
        }
    }

    pub fn pause(&mut self) -> Result<(), RunEngineError> {
        self.state = self.state.pause()?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), RunEngineError> {
        self.state = self.state.resume()?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), RunEngineError> {
        self.state = self.state.stop()?.complete_stop()?;
        Ok(())
    }

    pub fn step_one_paused(&mut self) -> Result<(), RunEngineError> {
        self.state.validate_step_run()?;
        self.commit_one_tick()
    }

    pub fn run_one_tick(&mut self) -> Result<(), RunEngineError> {
        match self.state {
            ActiveRunState::Running => self.commit_one_tick(),
            _ => Err(RunEngineError::StateConflict),
        }
    }

    pub fn run_until_configured_tick(&mut self) -> Result<(), RunEngineError> {
        while self.current_tick() < self.max_ticks {
            self.run_one_tick()?;
        }
        self.state = ActiveRunState::Completed;
        Ok(())
    }

    pub fn current_tick(&self) -> u64 {
        self.executor
            .as_ref()
            .map(|executor| executor.world().tick().raw())
            .unwrap_or(0)
    }

    pub const fn max_ticks(&self) -> u64 {
        self.max_ticks
    }

    pub fn scenario_hash(&self) -> Option<ScenarioHash> {
        self.scenario_hash
    }

    pub fn snapshots(&self) -> &RingBuffer<CommittedSnapshot> {
        &self.snapshots
    }

    pub fn latest_committed_snapshot(&mut self) -> CommittedSnapshot {
        let executor = self.executor.as_ref().expect("run engine is prepared");
        self.snapshot_build_count += 1;
        CommittedSnapshot::from_world(executor.world())
    }

    pub const fn snapshot_build_count_for_test(&self) -> u64 {
        self.snapshot_build_count
    }

    fn commit_one_tick(&mut self) -> Result<(), RunEngineError> {
        let executor = self.executor.as_mut().ok_or(RunEngineError::NotPrepared)?;
        executor.step()?;
        let committed_tick = executor.world().tick().raw();
        if self
            .config
            .snapshot_cadence
            .should_cache_after_tick(committed_tick)
        {
            self.snapshots
                .push(CommittedSnapshot::from_world(executor.world()));
            self.snapshot_build_count += 1;
        }
        Ok(())
    }
}
