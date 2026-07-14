use alife::bootstrap::prepare;
use alife::core::snapshot::CommittedSnapshot;
use alife::core::tick::TickExecutor;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

#[test]
fn minimal_viable_world_bootstraps_and_core_advances_after_start() {
    let path = "config/scenarios/bootstrap/minimal_viable_world.toml";
    let content = std::fs::read_to_string(path).unwrap();
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: path.to_string(),
        content,
    })
    .unwrap();

    let prepared = prepare(&document).unwrap();
    let prepared_again = prepare(&document).unwrap();
    assert_eq!(
        prepared.prepared_state_hash,
        prepared_again.prepared_state_hash
    );

    let mut executor = TickExecutor::new(prepared.runtime_config).unwrap();
    assert_eq!(executor.world().tick().raw(), 0);

    executor.step().unwrap();
    executor.step().unwrap();

    let snapshot = CommittedSnapshot::from_world(executor.world());
    assert_eq!(snapshot.tick.raw(), 2);
    assert!(!snapshot.cells.is_empty());
}

#[test]
fn same_seed_produces_same_short_smoke_summary() {
    let path = "config/scenarios/bootstrap/minimal_viable_world.toml";
    let content = std::fs::read_to_string(path).unwrap();
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: path.to_string(),
        content,
    })
    .unwrap();

    let run = || {
        let prepared = prepare(&document).unwrap();
        let mut executor = TickExecutor::new(prepared.runtime_config).unwrap();
        executor.step().unwrap();
        executor.step().unwrap();
        let snapshot = CommittedSnapshot::from_world(executor.world());
        (
            snapshot.tick.raw(),
            snapshot.cells.len(),
            snapshot.heat.to_bits(),
            snapshot.waste.to_bits(),
        )
    };

    assert_eq!(run(), run());
}
