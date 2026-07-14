use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::lifecycle::ActiveRunState;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

#[test]
fn run_engine_prepares_from_scenario_document_via_bootstrap_without_tick() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();

    let engine = RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();

    assert_eq!(engine.state(), ActiveRunState::Paused);
    assert_eq!(engine.current_tick(), 0);
    assert_eq!(engine.scenario_hash().unwrap(), document.scenario_hash);
}

#[test]
fn start_run_advances_from_paused_prepared_world_to_running() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();

    engine.start().unwrap();

    assert_eq!(engine.state(), ActiveRunState::Running);
    assert_eq!(engine.current_tick(), 0);
}
