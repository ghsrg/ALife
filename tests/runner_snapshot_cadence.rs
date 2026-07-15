use alife::runner::engine::{RunEngine, RunEngineConfig, SnapshotCadence};
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
            snapshot_cadence: SnapshotCadence::EveryNTicks(5),
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

#[test]
fn snapshot_cadence_on_demand_only_never_builds_tick_cache_snapshots() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 4,
            snapshot_cadence: SnapshotCadence::OnDemandOnly,
        },
    )
    .unwrap();

    assert_eq!(engine.snapshot_build_count_for_test(), 1);
    assert_eq!(engine.snapshots().len(), 1);

    engine.start().unwrap();
    for _ in 0..10 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 10);
    assert_eq!(engine.snapshots().len(), 1);
    assert_eq!(engine.snapshot_build_count_for_test(), 1);

    let snapshot = engine.latest_committed_snapshot();
    assert_eq!(snapshot.tick.raw(), 10);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);
}

#[test]
fn snapshot_cadence_every_n_ticks_replaces_legacy_numeric_field() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 8,
            snapshot_cadence: SnapshotCadence::EveryNTicks(5),
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

    engine.run_one_tick().unwrap();
    assert_eq!(engine.current_tick(), 5);
    assert_eq!(engine.snapshots().len(), 2);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);
}

#[test]
fn headless_debug_config_uses_on_demand_snapshots() {
    let config = RunEngineConfig::headless_debug();

    assert_eq!(config.snapshot_buffer_size, 4);
    assert_eq!(config.snapshot_cadence, SnapshotCadence::OnDemandOnly);
}
