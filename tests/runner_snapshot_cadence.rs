use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn document(id: &str) -> alife::runner::scenario_doc::ScenarioDocument {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    load_scenario_document(meta).unwrap()
}

#[test]
fn run_engine_keeps_latest_committed_state_without_snapshotting_every_tick() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 8,
            snapshot_every_ticks: 5,
        },
    )
    .unwrap();

    engine.start().unwrap();
    for _ in 0..4 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 4);
    assert_eq!(engine.snapshots().len(), 1);
    assert_eq!(engine.snapshot_build_count_for_test(), 1);
    assert_eq!(engine.latest_committed_snapshot().tick.raw(), 4);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);

    engine.run_one_tick().unwrap();
    assert_eq!(engine.current_tick(), 5);
    assert_eq!(engine.snapshots().len(), 2);
    assert_eq!(engine.snapshot_build_count_for_test(), 3);
}
