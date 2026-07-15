use alife::core::cell_store::LifecycleState;
use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::projections::WorldFrameProjection;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};
use std::path::PathBuf;

fn bootstrap_document() -> alife::runner::scenario_doc::ScenarioDocument {
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios");
    let meta = scan_scenarios(&scenarios_dir)
        .expect("scenarios should scan")
        .into_iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .expect("bootstrap_minimal_viable_world should exist");
    load_scenario_document(&meta).expect("scenario document should load")
}

#[test]
fn world_frame_projection_declares_schema_version_and_tick() {
    let document = bootstrap_document();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();
    engine.start().unwrap();
    engine.run_one_tick().unwrap();
    let snapshot = engine.snapshots().newest().expect("snapshot should exist");

    let projection = WorldFrameProjection::from_committed_snapshot(snapshot);

    assert_eq!(projection.schema_version, 2);
    assert_eq!(projection.committed_tick, engine.current_tick());
}

#[test]
fn world_frame_projection_includes_interpolation_metadata() {
    let document = bootstrap_document();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();
    engine.start().unwrap();
    engine.run_one_tick().unwrap();
    let snapshot = engine.snapshots().newest().expect("snapshot should exist");

    let projection = WorldFrameProjection::from_committed_snapshot_with_metadata(
        snapshot,
        7,
        1_725_000_000_000,
        Some(0),
    );

    assert_eq!(projection.projection_sequence, 7);
    assert_eq!(projection.wall_clock_generated_at_ms, 1_725_000_000_000);
    assert_eq!(projection.previous_committed_tick, Some(0));
}

#[test]
fn world_frame_projection_maps_visible_cell_fields() {
    let document = bootstrap_document();
    let mut engine =
        RunEngine::prepare_from_document(&document, RunEngineConfig::default()).unwrap();
    engine.start().unwrap();
    engine.run_one_tick().unwrap();
    let snapshot = engine.snapshots().newest().expect("snapshot should exist");

    let projection = WorldFrameProjection::from_committed_snapshot(snapshot);
    let projected = projection
        .cells
        .first()
        .expect("bootstrap scenario should project at least one cell");
    let source = snapshot
        .cells
        .first()
        .expect("bootstrap scenario should snapshot at least one cell");

    assert_eq!(projected.id, source.id.raw());
    assert_eq!(projected.x, source.position.x());
    assert_eq!(projected.y, source.position.y());
    assert_eq!(projected.radius, source.radius.raw());
    assert_eq!(projected.energy, source.energy.raw());
    assert_eq!(
        projected.lifecycle,
        match source.lifecycle_state {
            LifecycleState::Alive => 0,
            LifecycleState::Stressed => 1,
            LifecycleState::Dormant => 2,
            LifecycleState::Dead => 3,
        }
    );
}
