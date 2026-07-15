use crate::runner::commands::RunnerCommand;
use crate::runner::engine::RunEngine;
use crate::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use crate::runner::projections::WorldFrameProjection;
use crate::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use crate::viewer_server::broadcaster::{Broadcaster, WsMessage};
use crate::viewer_server::frame_encoder::encode_world_frame;
use crate::viewer_server::projection_sampler::{
    ProjectionDecision, ViewerProjectionConfig, ViewerProjectionSampler,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct TickLoopSignal {
    stop: Arc<AtomicBool>,
    pause: Arc<AtomicBool>,
}

impl TickLoopSignal {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            pause: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn request_pause(&self) {
        self.pause.store(true, Ordering::Relaxed);
    }

    pub fn request_resume(&self) {
        self.pause.store(false, Ordering::Relaxed);
    }

    pub fn is_stop_requested(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn is_pause_requested(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }
}

impl Default for TickLoopSignal {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RunnerCommandResult {
    pub process_state: RunnerProcessState,
    pub active_run_state: ActiveRunState,
    pub run_id: Option<String>,
    pub committed_tick: u64,
    pub scenario_hash: Option<String>,
    pub effective_seed: Option<u64>,
    pub terminal_reason: Option<String>,
}

pub struct SharedState {
    pub engine: Option<RunEngine>,
    pub process_state: RunnerProcessState,
    pub active_run_state: ActiveRunState,
    pub run_id: Option<String>,
    pub scenario_hash: Option<String>,
    pub effective_seed: Option<u64>,
    pub scenario_id: Option<String>,
    pub committed_tick: u64,
    pub terminal_reason: Option<String>,
    pub scenarios_dir: PathBuf,
    pub tick_signal: Option<Arc<TickLoopSignal>>,
    pub engine_snapshot_buffer_size: usize,
    pub target_broadcast_fps: u32,
    pub viewer_projection: ViewerProjectionConfig,
    pub broadcaster: Broadcaster,
}

impl SharedState {
    pub fn new(
        scenarios_dir: PathBuf,
        engine_snapshot_buffer_size: usize,
        target_broadcast_fps: u32,
        viewer_projection: ViewerProjectionConfig,
    ) -> Self {
        Self {
            engine: None,
            process_state: RunnerProcessState::Ready,
            active_run_state: ActiveRunState::Idle,
            run_id: None,
            scenario_hash: None,
            effective_seed: None,
            scenario_id: None,
            committed_tick: 0,
            terminal_reason: None,
            scenarios_dir,
            tick_signal: None,
            engine_snapshot_buffer_size,
            target_broadcast_fps,
            viewer_projection,
            broadcaster: Broadcaster::new(128),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.active_run_state,
            ActiveRunState::Preparing
                | ActiveRunState::Running
                | ActiveRunState::Paused
                | ActiveRunState::Stopping
        )
    }

    pub fn status_projection(&self) -> RunnerCommandResult {
        RunnerCommandResult {
            process_state: self.process_state,
            active_run_state: self.active_run_state,
            run_id: self.run_id.clone(),
            committed_tick: self.committed_tick,
            scenario_hash: self.scenario_hash.clone(),
            effective_seed: self.effective_seed,
            terminal_reason: self.terminal_reason.clone(),
        }
    }
}

pub type AppState = Arc<Mutex<SharedState>>;

pub fn process_state_label(state: RunnerProcessState) -> &'static str {
    match state {
        RunnerProcessState::Starting => "starting",
        RunnerProcessState::Ready => "ready",
        RunnerProcessState::ShuttingDown => "shutting_down",
        RunnerProcessState::Failed => "failed",
    }
}

pub fn active_state_label(state: ActiveRunState) -> &'static str {
    match state {
        ActiveRunState::Idle => "idle",
        ActiveRunState::Preparing => "preparing",
        ActiveRunState::Running => "running",
        ActiveRunState::Paused => "paused",
        ActiveRunState::Stopping => "stopping",
        ActiveRunState::Completed => "completed",
        ActiveRunState::Failed => "failed",
    }
}

pub fn status_ws_text_from_state(state: &SharedState) -> String {
    serde_json::json!({
        "type": "status",
        "process_state": process_state_label(state.process_state),
        "active_run_state": active_state_label(state.active_run_state),
        "run_id": state.run_id,
        "committed_tick": state.committed_tick,
        "scenario_id": state.scenario_id,
        "scenario_hash": state.scenario_hash,
        "effective_seed": state.effective_seed,
        "terminal_reason": state.terminal_reason,
    })
    .to_string()
}

pub fn encode_latest_frame(state: &mut SharedState) -> Option<Vec<u8>> {
    let engine = state.engine.as_mut()?;
    let snapshot = engine.latest_committed_snapshot();
    let projection = WorldFrameProjection::from_committed_snapshot(&snapshot);
    Some(encode_world_frame(&projection))
}

pub fn new_app_state(
    scenarios_dir: PathBuf,
    engine_snapshot_buffer_size: usize,
    target_broadcast_fps: u32,
) -> AppState {
    new_app_state_with_projection(
        scenarios_dir,
        engine_snapshot_buffer_size,
        target_broadcast_fps,
        ViewerProjectionConfig {
            target_frames_per_second: target_broadcast_fps,
            ..ViewerProjectionConfig::default()
        },
    )
}

pub fn new_app_state_with_projection(
    scenarios_dir: PathBuf,
    engine_snapshot_buffer_size: usize,
    target_broadcast_fps: u32,
    viewer_projection: ViewerProjectionConfig,
) -> AppState {
    Arc::new(Mutex::new(SharedState::new(
        scenarios_dir,
        engine_snapshot_buffer_size,
        target_broadcast_fps,
        viewer_projection,
    )))
}

pub fn resolve_scenario_document(
    state: &SharedState,
    scenario_id_or_path: &str,
) -> Result<ScenarioDocument, String> {
    let candidate = Path::new(scenario_id_or_path);
    let source_path = if candidate.exists() {
        candidate.to_path_buf()
    } else {
        state
            .scenarios_dir
            .join(format!("{scenario_id_or_path}.toml"))
    };
    ScenarioDocument::resolve(ScenarioSource::Path(source_path)).map_err(|err| err.to_string())
}

pub fn dispatch_command(
    state: &AppState,
    command: RunnerCommand,
) -> Result<RunnerCommandResult, String> {
    let mut locked = state.lock().unwrap();
    command
        .validate(locked.active_run_state)
        .map_err(|_| "state_conflict".to_string())?;

    match command {
        RunnerCommand::GetRunStatus => Ok(locked.status_projection()),
        RunnerCommand::PauseRun => {
            {
                let engine = locked
                    .engine
                    .as_mut()
                    .ok_or_else(|| "run_not_found".to_string())?;
                engine.pause().map_err(|err| err.to_string())?;
            }
            locked.active_run_state = ActiveRunState::Paused;
            if let Some(signal) = &locked.tick_signal {
                signal.request_pause();
            }
            Ok(locked.status_projection())
        }
        RunnerCommand::ResumeRun => {
            {
                let engine = locked
                    .engine
                    .as_mut()
                    .ok_or_else(|| "run_not_found".to_string())?;
                engine.resume().map_err(|err| err.to_string())?;
            }
            locked.active_run_state = ActiveRunState::Running;
            if let Some(signal) = &locked.tick_signal {
                signal.request_resume();
            }
            Ok(locked.status_projection())
        }
        RunnerCommand::StepRun => {
            let committed_tick = {
                let engine = locked
                    .engine
                    .as_mut()
                    .ok_or_else(|| "run_not_found".to_string())?;
                engine.step_one_paused().map_err(|err| err.to_string())?;
                engine.current_tick()
            };
            locked.committed_tick = committed_tick;
            locked.active_run_state = ActiveRunState::Paused;
            Ok(locked.status_projection())
        }
        RunnerCommand::StopRun => {
            if let Some(signal) = &locked.tick_signal {
                signal.request_stop();
            }
            if let Some(engine) = locked.engine.as_mut() {
                engine.stop().map_err(|err| err.to_string())?;
            }
            locked.active_run_state = ActiveRunState::Completed;
            locked.engine = None;
            locked.tick_signal = None;
            Ok(locked.status_projection())
        }
        RunnerCommand::ValidateScenario
        | RunnerCommand::PrepareScenario
        | RunnerCommand::StartRun => {
            Err("use dedicated handler because this command requires scenario input".to_string())
        }
    }
}

pub fn spawn_tick_loop(state: AppState) {
    let signal = Arc::new(TickLoopSignal::new());
    let (broadcast_sender, viewer_projection) = {
        let locked = state.lock().unwrap();
        (locked.broadcaster.sender(), locked.viewer_projection)
    };
    {
        let mut locked = state.lock().unwrap();
        locked.tick_signal = Some(Arc::clone(&signal));
    }

    std::thread::spawn(move || {
        let mut sampler = ViewerProjectionSampler::new(viewer_projection);
        loop {
            if signal.is_stop_requested() {
                break;
            }
            if signal.is_pause_requested() {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }

            let result = {
                let mut locked = state.lock().unwrap();
                let tick_result = match locked.engine.as_mut() {
                    Some(engine) => match engine.run_one_tick() {
                        Ok(()) => {
                            let committed_tick = engine.current_tick();
                            let frame = match sampler.on_committed_tick(committed_tick, Instant::now()) {
                                ProjectionDecision::Emit
                                | ProjectionDecision::EmitForced
                                | ProjectionDecision::EmitHeartbeat => {
                                    let snapshot = engine.latest_committed_snapshot();
                                    let projection =
                                        WorldFrameProjection::from_committed_snapshot(&snapshot);
                                    Some(encode_world_frame(&projection))
                                }
                                ProjectionDecision::Skip => None,
                            };
                            Ok((committed_tick, frame))
                        }
                        Err(err) => Err(err.to_string()),
                    },
                    None => break,
                };

                match tick_result {
                    Ok((committed_tick, frame)) => {
                        locked.committed_tick = committed_tick;
                        Ok(frame)
                    }
                    Err(err) => Err(err),
                }
            };

            match result {
                Ok(Some(bytes)) => {
                    let _ = broadcast_sender.send(WsMessage::Frame(bytes));
                }
                Ok(None) => {}
                Err(_) => {
                    let mut locked = state.lock().unwrap();
                    locked.active_run_state = ActiveRunState::Failed;
                    locked.terminal_reason = Some("core_error".to_string());
                    break;
                }
            }
        }

        let mut locked = state.lock().unwrap();
        if matches!(locked.active_run_state, ActiveRunState::Running) {
            locked.active_run_state = ActiveRunState::Completed;
        }
        locked.tick_signal = None;
    });
}
