use alife::core::cell_store::LifecycleState;
use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::lifecycle::ActiveRunState;
use alife::runner::progress::{ProgressInterval, ProgressSnapshot, format_progress_table};
use alife::runner::scenario::{ScenarioMeta, load_scenario_document, scan_scenarios};
use alife::runner::server_config::{ServerConfig, load_server_config};
use alife::viewer_server::{create_app, state::new_app_state_with_projection};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone)]
struct RunnerCli {
    scenario: Option<String>,
    list: bool,
    serve: bool,
    debug: bool,
    progress_interval: ProgressInterval,
}

fn parse_cli(args: &[String]) -> Result<RunnerCli, String> {
    let mut cli = RunnerCli {
        scenario: None,
        list: false,
        serve: false,
        debug: false,
        progress_interval: ProgressInterval::default(),
    };
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--list" => cli.list = true,
            "--serve" => cli.serve = true,
            "--debug" => cli.debug = true,
            "--progress-interval-ms" => {
                index += 1;
                let raw = args
                    .get(index)
                    .ok_or_else(|| "--progress-interval-ms requires a value".to_string())?;
                let ms = raw
                    .parse::<u64>()
                    .map_err(|_| "--progress-interval-ms must be an integer".to_string())?;
                cli.progress_interval =
                    ProgressInterval::from_millis(ms).map_err(ToOwned::to_owned)?;
            }
            value if value.starts_with("--") => return Err(format!("unknown flag: {value}")),
            value => {
                if cli.scenario.is_some() {
                    return Err("only one scenario path or id is supported".to_string());
                }
                cli.scenario = Some(value.to_string());
            }
        }
        index += 1;
    }
    Ok(cli)
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let cli = parse_cli(&args).unwrap_or_else(|err| {
        eprintln!("[runner] {err}");
        eprintln!(
            "Usage: runner [--debug] [--progress-interval-ms N] <scenario-id-or-path> | --list | --serve"
        );
        std::process::exit(2);
    });

    let scenarios_dir = PathBuf::from("config/scenarios");
    if cli.list {
        list_scenarios(&scenarios_dir);
        return;
    }
    if cli.serve {
        let server_config_path = PathBuf::from("config/server.toml");
        let server_config = load_server_config(&server_config_path).unwrap_or_else(|err| {
            eprintln!("[runner] {err}");
            std::process::exit(1);
        });
        if let Err(err) = serve_http(server_config, scenarios_dir).await {
            eprintln!("[runner] {err}");
            std::process::exit(1);
        }
        return;
    }
    let Some(scenario) = cli.scenario.as_deref() else {
        eprintln!(
            "Usage: runner [--debug] [--progress-interval-ms N] <scenario-id-or-path> | --list | --serve"
        );
        std::process::exit(2);
    };

    if let Err(err) = run_headless(scenario, &scenarios_dir, cli.debug, cli.progress_interval) {
        eprintln!("[runner] {err}");
        std::process::exit(1);
    }
}

async fn serve_http(server_config: ServerConfig, scenarios_dir: PathBuf) -> Result<(), String> {
    let bind_addr = server_config.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .map_err(|err| format!("failed to bind HTTP server on {bind_addr}: {err}"))?;
    let app_state = new_app_state_with_projection(
        scenarios_dir,
        300,
        server_config.target_broadcast_fps,
        server_config.viewer_projection,
    );
    let app = create_app(app_state);

    println!("[runner] HTTP server listening on http://{bind_addr}");
    println!("[runner] GET  /server/info");
    println!("[runner] GET  /scenarios");
    println!("[runner] GET  /run/status");
    println!("[runner] POST /run/start");

    axum::serve(listener, app)
        .await
        .map_err(|err| format!("HTTP server failed: {err}"))
}

fn list_scenarios(scenarios_dir: &Path) {
    match scan_scenarios(scenarios_dir) {
        Ok(scenarios) => {
            println!("Available scenarios in {}:", scenarios_dir.display());
            for scenario in scenarios {
                println!("  {} ({})", scenario.id, scenario.path.display());
            }
        }
        Err(err) => {
            eprintln!("[runner] failed to scan scenarios: {err}");
            std::process::exit(1);
        }
    }
}

fn run_headless(
    scenario: &str,
    scenarios_dir: &Path,
    debug: bool,
    progress_interval: ProgressInterval,
) -> Result<(), String> {
    let meta = resolve_scenario(scenario, scenarios_dir)?;
    println!(
        "[runner] Loading scenario: {} ({})",
        meta.id,
        meta.path.display()
    );
    let document = load_scenario_document(&meta).map_err(|err| err.to_string())?;
    let engine_config = if debug {
        RunEngineConfig::headless_debug()
    } else {
        RunEngineConfig::default()
    };
    let mut engine = RunEngine::prepare_from_document(&document, engine_config)
        .map_err(|err| err.to_string())?;
    println!("[runner] Prepared scenario_hash={}", document.scenario_hash);
    engine.start().map_err(|err| err.to_string())?;

    let start = Instant::now();
    println!("[runner] Running {} ticks...", engine.max_ticks());
    if debug {
        let mut next_progress_at = start + progress_interval.as_duration();
        while engine.current_tick() < engine.max_ticks() {
            engine.run_one_tick().map_err(|err| err.to_string())?;
            let now = Instant::now();
            if engine.current_tick() == 1 || now >= next_progress_at {
                print_progress(&mut engine, start);
                next_progress_at = now + progress_interval.as_duration();
            }
        }
    } else {
        engine
            .run_until_configured_tick()
            .map_err(|err| err.to_string())?;
    }

    let elapsed = start.elapsed().as_secs_f32();
    let tps = if elapsed > 0.0 {
        engine.current_tick() as f32 / elapsed
    } else {
        0.0
    };
    println!(
        "[runner] Completed {} ticks in {:.2}s ({:.0} ticks/sec)",
        engine.current_tick(),
        elapsed,
        tps
    );
    let snapshot = engine.latest_committed_snapshot();
    println!(
        "[runner] Final tick: {}, cells: {}, heat: {:.2}, waste: {:.2}",
        snapshot.tick.raw(),
        snapshot.cells.len(),
        snapshot.heat,
        snapshot.waste
    );
    Ok(())
}

fn resolve_scenario(scenario: &str, scenarios_dir: &Path) -> Result<ScenarioMeta, String> {
    let path = PathBuf::from(scenario);
    if path.exists() {
        let document = alife::runner::scenario_doc::ScenarioDocument::resolve(
            alife::runner::scenario_doc::ScenarioSource::Path(path.clone()),
        )
        .map_err(|err| err.to_string())?;
        return Ok(ScenarioMeta {
            id: document.id,
            path,
        });
    }
    scan_scenarios(scenarios_dir)
        .map_err(|err| err.to_string())?
        .into_iter()
        .find(|meta| meta.id == scenario)
        .ok_or_else(|| format!("scenario not found: {scenario}"))
}

fn print_progress(engine: &mut RunEngine, start: Instant) {
    let snapshot = engine.latest_committed_snapshot();
    let elapsed = start.elapsed();
    let elapsed_s = elapsed.as_secs_f32();
    let alive = snapshot
        .cells
        .iter()
        .filter(|cell| cell.lifecycle_state != LifecycleState::Dead)
        .count();
    let dead = snapshot.cells.len().saturating_sub(alive);
    let progress = ProgressSnapshot {
        elapsed_ms: elapsed.as_millis(),
        tick: engine.current_tick(),
        max_ticks: engine.max_ticks(),
        ticks_per_second: if elapsed_s > 0.0 {
            engine.current_tick() as f32 / elapsed_s
        } else {
            0.0
        },
        cells: snapshot.cells.len(),
        alive_cells: Some(alive),
        dead_cells: Some(dead),
        heat: snapshot.heat,
        waste: snapshot.waste,
        state: format!("{:?}", engine.state()),
        collapse_reason: if engine.state() == ActiveRunState::Failed {
            Some("failed".to_string())
        } else {
            None
        },
    };
    println!("{}", format_progress_table(&progress));
}
