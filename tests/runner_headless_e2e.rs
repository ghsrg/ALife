use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::lifecycle::ActiveRunState;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn engine() -> RunEngine {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();
    engine.start().unwrap();
    engine
}

#[test]
fn headless_run_reaches_configured_tick_and_completes() {
    let mut engine = engine();

    engine.run_until_configured_tick().unwrap();

    assert_eq!(engine.state(), ActiveRunState::Completed);
    assert_eq!(engine.current_tick(), engine.max_ticks());
    assert!(engine.snapshots().newest().is_some());
}

#[test]
fn same_seed_produces_same_final_snapshot_summary() {
    let run = || {
        let mut engine = engine();
        engine.run_until_configured_tick().unwrap();
        let snap = engine.snapshots().newest().unwrap();
        (
            snap.tick.raw(),
            snap.cells.len(),
            snap.heat.to_bits(),
            snap.waste.to_bits(),
        )
    };

    assert_eq!(run(), run());
}

#[test]
fn step_run_executes_one_tick_only_while_paused() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();

    engine.step_one_paused().unwrap();

    assert_eq!(engine.state(), ActiveRunState::Paused);
    assert_eq!(engine.current_tick(), 1);
}
