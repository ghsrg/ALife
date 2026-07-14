use crate::core::units::Seed;
use crate::runner::commands::RunnerCommand;
use crate::runner::engine::{RunEngine, RunEngineConfig};
use crate::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use crate::runner::scenario::{load_scenario_document, scan_scenarios};
use crate::runner::scenario_doc::ScenarioDocument;
use crate::viewer_server::state::{AppState, dispatch_command, spawn_tick_loop};
use axum::{
    Json, Router,
    extract::{Json as ExtractJson, State},
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
    }
}

fn error_response(
    status: StatusCode,
    category: &str,
    message: impl Into<String>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            ok: false,
            category: category.to_string(),
            message: message.into(),
            current_state: "see /run/status".to_string(),
        }),
    )
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
    ExtractJson(req): ExtractJson<StartRequest>,
) -> Result<Json<StartResponse>, (StatusCode, Json<ErrorResponse>)> {
    {
        let locked = state.lock().unwrap();
        if locked.is_active() {
            return Err(error_response(
                StatusCode::CONFLICT,
                "state_conflict",
                format!("Run already active in {:?}", locked.active_run_state),
            ));
        }
    }

    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let meta = scan_scenarios(&scenarios_dir)
        .map_err(|err| error_response(StatusCode::BAD_REQUEST, "scenario_error", err.to_string()))?
        .into_iter()
        .find(|scenario| scenario.id == req.scenario_id)
        .ok_or_else(|| {
            error_response(
                StatusCode::BAD_REQUEST,
                "scenario_error",
                format!("Scenario not found: {}", req.scenario_id),
            )
        })?;
    let document = load_scenario_document(&meta).map_err(|err| {
        error_response(StatusCode::BAD_REQUEST, "scenario_error", err.to_string())
    })?;
    let (document, effective_seed) = document_with_seed_override(document, req.seed_override);
    let scenario_hash = document.scenario_hash.to_string();
    let mut engine = RunEngine::prepare_from_document(
        &document,
        RunEngineConfig {
            snapshot_buffer_size: state.lock().unwrap().engine_snapshot_buffer_size,
        },
    )
    .map_err(|err| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "bootstrap_error",
            err.to_string(),
        )
    })?;
    engine.start().map_err(|err| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "core_error",
            err.to_string(),
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
    }

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
    dispatch_command(&state, RunnerCommand::PauseRun)
        .map(command_result_response)
        .map_err(|category| error_response(StatusCode::CONFLICT, &category, "Cannot pause run"))
}

async fn handle_run_resume(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    dispatch_command(&state, RunnerCommand::ResumeRun)
        .map(command_result_response)
        .map_err(|category| error_response(StatusCode::CONFLICT, &category, "Cannot resume run"))
}

async fn handle_run_step(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    dispatch_command(&state, RunnerCommand::StepRun)
        .map(command_result_response)
        .map_err(|category| error_response(StatusCode::CONFLICT, &category, "Cannot step run"))
}

async fn handle_run_stop(
    State(state): State<AppState>,
) -> Result<Json<CommandResponse>, (StatusCode, Json<ErrorResponse>)> {
    dispatch_command(&state, RunnerCommand::StopRun)
        .map(command_result_response)
        .map_err(|category| error_response(StatusCode::CONFLICT, &category, "Cannot stop run"))
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
