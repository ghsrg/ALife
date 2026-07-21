use crate::core::units::Seed;
use crate::runner::commands::RunnerCommand;
use crate::runner::engine::{RunEngine, RunEngineConfig, SnapshotCadence};
use crate::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use crate::runner::scenario::{load_scenario_document, scan_scenarios};
use crate::runner::scenario_doc::ScenarioDocument;
use crate::viewer_server::broadcaster::WsMessage;
use crate::viewer_server::state::{
    AppState, dispatch_command, encode_latest_frame, spawn_tick_loop, status_ws_text_from_state,
};
use axum::{
    Json, Router,
    extract::{Json as ExtractJson, State, rejection::JsonRejection},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RunStatus {
    process_state: String,
    active_run_state: String,
    run_id: Option<String>,
    committed_tick: u64,
    scenario_id: Option<String>,
    scenario_hash: Option<String>,
    effective_seed: Option<u64>,
    terminal_reason: Option<String>,
    collapse_reason: Option<String>,
    ticks_per_second: f32,
}

#[derive(Deserialize)]
struct StartRequest {
    scenario_id: String,
    seed_override: Option<u64>,
    request_id: Option<String>,
}

#[derive(Serialize)]
struct StartResponse {
    ok: bool,
    run_id: String,
    scenario_hash: String,
    effective_seed: u64,
    active_run_state: String,
    bootstrap_manifest: serde_json::Value,
}

#[derive(Serialize)]
struct CommandResponse {
    ok: bool,
    active_run_state: String,
    committed_tick: u64,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    category: String,
    message: String,
    command: String,
    scenario_id: Option<String>,
    process_state: String,
    active_run_state: String,
    current_state: String,
}

fn process_state_label(state: RunnerProcessState) -> &'static str {
    match state {
        RunnerProcessState::Starting => "starting",
        RunnerProcessState::Ready => "ready",
        RunnerProcessState::ShuttingDown => "shutting_down",
        RunnerProcessState::Failed => "failed",
    }
}

fn active_state_label(state: ActiveRunState) -> &'static str {
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

fn status_from_state(state: &crate::viewer_server::state::SharedState) -> RunStatus {
    RunStatus {
        process_state: process_state_label(state.process_state).to_string(),
        active_run_state: active_state_label(state.active_run_state).to_string(),
        run_id: state.run_id.clone(),
        committed_tick: state.committed_tick,
        scenario_id: state.scenario_id.clone(),
        scenario_hash: state.scenario_hash.clone(),
        effective_seed: state.effective_seed,
        terminal_reason: state.terminal_reason.clone(),
        collapse_reason: state.terminal_reason.clone(),
        ticks_per_second: state.ticks_per_second(),
    }
}

fn error_response(
    state: &AppState,
    status: StatusCode,
    category: &str,
    message: impl Into<String>,
    command: &str,
    scenario_id: Option<String>,
) -> (StatusCode, Json<ErrorResponse>) {
    let locked = state.lock().unwrap();
    let process_state = process_state_label(locked.process_state).to_string();
    let active_run_state = active_state_label(locked.active_run_state).to_string();
    (
        status,
        Json(ErrorResponse {
            ok: false,
            category: category.to_string(),
            message: message.into(),
            command: command.to_string(),
            scenario_id,
            process_state: process_state.clone(),
            active_run_state: active_run_state.clone(),
            current_state: format!("{process_state}/{active_run_state}"),
        }),
    )
}

fn broadcast_status(state: &AppState) {
    let (text, broadcaster) = {
        let locked = state.lock().unwrap();
        (
            status_ws_text_from_state(&locked),
            locked.broadcaster.sender(),
        )
    };
    let _ = broadcaster.send(crate::viewer_server::broadcaster::WsMessage::Status(text));
}

fn broadcast_forced_frame(state: &AppState) {
    let (sender, frame) = {
        let mut locked = state.lock().unwrap();
        (
            locked.broadcaster.sender(),
            encode_latest_frame(&mut locked),
        )
    };
    if let Some(bytes) = frame {
        let _ = sender.send(WsMessage::Frame(bytes));
    }
}

async fn handle_run_status(State(state): State<AppState>) -> Json<RunStatus> {
    let _ = dispatch_command(&state, RunnerCommand::GetRunStatus);
    let locked = state.lock().unwrap();
    Json(status_from_state(&locked))
}

fn document_with_seed_override(
    document: ScenarioDocument,
    seed_override: Option<u64>,
) -> (ScenarioDocument, u64) {
    let Some(seed) = seed_override else {
        return (document.clone(), document.runtime_config.world.seed.raw());
    };

    let mut runtime_config = document.runtime_config.clone();
    runtime_config.world.seed = Seed::from_raw(seed);
    let canonical_source = document
        .canonical_source
        .lines()
        .map(|line| {
            if line.starts_with("seed=") {
                format!("seed={seed}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    (
        ScenarioDocument::from_runtime_config(document.id, runtime_config, canonical_source),
        seed,
    )
}

async fn handle_run_start(
    State(state): State<AppState>,
    req: Result<ExtractJson<StartRequest>, JsonRejection>,
) -> Result<Json<StartResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ExtractJson(req) = req.map_err(|err| {
        error_response(
            &state,
            StatusCode::BAD_REQUEST,
            "invalid_command",
            err.to_string(),
            "StartRun",
            None,
        )
    })?;
    let start_guard = {
        let locked = state.lock().unwrap();
        if matches!(locked.process_state, RunnerProcessState::ShuttingDown) {
            Some((
                StatusCode::SERVICE_UNAVAILABLE,
                "unsupported_operation",
                "Runner is shutting down".to_string(),
            ))
        } else if locked.is_active() {
            Some((
                StatusCode::CONFLICT,
                "state_conflict",
                format!("Run already active in {:?}", locked.active_run_state),
            ))
        } else {
            None
        }
    };
    if let Some((status, category, message)) = start_guard {
        return Err(error_response(
            &state,
            status,
            category,
            message,
            "StartRun",
            Some(req.scenario_id.clone()),
        ));
    }

    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let meta = scan_scenarios(&scenarios_dir)
        .map_err(|err| {
            error_response(
                &state,
                StatusCode::BAD_REQUEST,
                "scenario_error",
                err.to_string(),
                "StartRun",
                Some(req.scenario_id.clone()),
            )
        })?
        .into_iter()
        .find(|scenario| scenario.id == req.scenario_id)
        .ok_or_else(|| {
            error_response(
                &state,
                StatusCode::BAD_REQUEST,
                "scenario_error",
                format!("Scenario not found: {}", req.scenario_id),
                "StartRun",
                Some(req.scenario_id.clone()),
            )
        })?;
    let document = load_scenario_document(&meta).map_err(|err| {
        error_response(
            &state,
            StatusCode::BAD_REQUEST,
            "scenario_error",
            err.to_string(),
            "StartRun",
            Some(req.scenario_id.clone()),
        )
    })?;
    let (document, effective_seed) = document_with_seed_override(document, req.seed_override);
    let scenario_hash = document.scenario_hash.to_string();
    let mut engine = RunEngine::prepare_from_document(
        &document,
        RunEngineConfig {
            snapshot_buffer_size: state.lock().unwrap().engine_snapshot_buffer_size,
            snapshot_cadence: SnapshotCadence::EveryTick,
        },
    )
    .map_err(|err| {
        error_response(
            &state,
            StatusCode::INTERNAL_SERVER_ERROR,
            "bootstrap_error",
            err.to_string(),
            "StartRun",
            Some(req.scenario_id.clone()),
        )
    })?;
    engine.start().map_err(|err| {
        error_response(
            &state,
            StatusCode::INTERNAL_SERVER_ERROR,
            "core_error",
            err.to_string(),
            "StartRun",
            Some(req.scenario_id.clone()),
        )
    })?;

    let run_id = req
        .request_id
        .unwrap_or_else(|| format!("run-{}", scenario_hash));
    {
        let mut locked = state.lock().unwrap();
        locked.engine = Some(engine);
        locked.active_run_state = ActiveRunState::Running;
        locked.run_id = Some(run_id.clone());
        locked.scenario_id = Some(req.scenario_id);
        locked.scenario_hash = Some(scenario_hash.clone());
        locked.effective_seed = Some(effective_seed);
        locked.committed_tick = 0;
        locked.terminal_reason = None;
        locked.started_at = Some(std::time::Instant::now());
        locked.latest_frame = None;
    }

    broadcast_status(&state);
    spawn_tick_loop(state);

    Ok(Json(StartResponse {
        ok: true,
        run_id,
        scenario_hash: scenario_hash.clone(),
        effective_seed,
        active_run_state: "running".to_string(),
        bootstrap_manifest: serde_json::json!({
            "scenario_hash": scenario_hash,
            "effective_seed": effective_seed,
            "source": "prepared_world"
        }),
    }))
}

fn command_result_response(
    projection: crate::viewer_server::state::RunnerCommandResult,
) -> Json<CommandResponse> {
    Json(CommandResponse {
        ok: true,
        active_run_state: active_state_label(projection.active_run_state).to_string(),
        committed_tick: projection.committed_tick,
    })
}

async fn handle_run_pause(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = dispatch_command(&state, RunnerCommand::PauseRun)
        .map(command_result_response)
        .map_err(|category| {
            let scenario_id = state.lock().unwrap().scenario_id.clone();
            error_response(
                &state,
                StatusCode::CONFLICT,
                &category,
                "Cannot pause run",
                "PauseRun",
                scenario_id,
            )
        })?;
    broadcast_status(&state);
    broadcast_forced_frame(&state);
    Ok(response)
}

async fn handle_run_resume(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = dispatch_command(&state, RunnerCommand::ResumeRun)
        .map(command_result_response)
        .map_err(|category| {
            let scenario_id = state.lock().unwrap().scenario_id.clone();
            error_response(
                &state,
                StatusCode::CONFLICT,
                &category,
                "Cannot resume run",
                "ResumeRun",
                scenario_id,
            )
        })?;
    broadcast_status(&state);
    Ok(response)
}

async fn handle_run_step(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = dispatch_command(&state, RunnerCommand::StepRun)
        .map(command_result_response)
        .map_err(|category| {
            let scenario_id = state.lock().unwrap().scenario_id.clone();
            error_response(
                &state,
                StatusCode::CONFLICT,
                &category,
                "Cannot step run",
                "StepRun",
                scenario_id,
            )
        })?;
    broadcast_status(&state);
    broadcast_forced_frame(&state);
    Ok(response)
}

async fn handle_run_stop(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    let response = dispatch_command(&state, RunnerCommand::StopRun)
        .map(command_result_response)
        .map_err(|category| {
            let scenario_id = state.lock().unwrap().scenario_id.clone();
            error_response(
                &state,
                StatusCode::CONFLICT,
                &category,
                "Cannot stop run",
                "StopRun",
                scenario_id,
            )
        })?;
    broadcast_status(&state);
    broadcast_forced_frame(&state);
    Ok(response)
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/run/status", get(handle_run_status))
        .route("/run/start", post(handle_run_start))
        .route("/run/pause", post(handle_run_pause))
        .route("/run/resume", post(handle_run_resume))
        .route("/run/step", post(handle_run_step))
        .route("/run/stop", post(handle_run_stop))
        .with_state(state)
}
