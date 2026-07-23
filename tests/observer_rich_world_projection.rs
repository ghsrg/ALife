use alife::bootstrap::prepare;
use alife::core::cell_store::LifecycleState;
use alife::core::snapshot::CommittedSnapshot;
use alife::core::tick::TickExecutor;
use alife::observer::projection::build_visual_world_projection;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

#[test]
fn visual_world_projection_exposes_resource_grid_and_cell_details_for_rich_demo() {
    let content = std::fs::read_to_string("config/scenarios/demo/demo_world_resource.toml")
        .expect("resource demo scenario");
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "demo_world_resource".to_string(),
        content,
    })
    .unwrap();
    let prepared = prepare(&document).unwrap();
    let mut executor = TickExecutor::new(prepared.runtime_config).unwrap();
    executor.step().unwrap();

    let snapshot = CommittedSnapshot::from_world(executor.world());
    let projection = build_visual_world_projection(&snapshot);

    assert!(
        projection.cells.len() >= 3,
        "rich demo must have multiple cells"
    );
    assert!(
        projection.resource_layers.len() >= 3,
        "resource demo must expose multiple typed resource layers"
    );
    assert!(
        projection.resource_layers[0].cells.len() >= 64,
        "resource layer must expose sampled/exact grid cells, not totals only"
    );
    assert!(
        projection.resource_layers[0]
            .cells
            .iter()
            .any(|cell| cell.amount > 0.0)
    );
    let projected_cell = &projection.cells[0];
    assert!(projected_cell.energy_capacity > projected_cell.energy);
    assert!(!projected_cell.materials.is_empty());
    assert!(!projected_cell.internal_resources.is_empty());
    assert_eq!(
        projected_cell.local_external_resources.len(),
        projection.resource_layers.len()
    );
    assert!(
        projected_cell
            .local_external_resources
            .iter()
            .any(|r| r.amount > 0.0)
    );
}

#[test]
fn rich_demo_keeps_a_visible_population_alive_for_initial_ui_observation_window() {
    let content = std::fs::read_to_string("config/scenarios/demo/demo_world_resource.toml")
        .expect("resource demo scenario");
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "demo_world_resource".to_string(),
        content,
    })
    .unwrap();
    let prepared = prepare(&document).unwrap();
    let mut executor = TickExecutor::new(prepared.runtime_config).unwrap();

    for _ in 0..100 {
        executor.step().unwrap();
    }

    let alive = (0..executor.world().cells().len())
        .filter(|index| {
            executor
                .world()
                .cells()
                .lifecycle_state(alife::core::cell_store::CellIndex::from_raw(*index))
                != LifecycleState::Dead
        })
        .count();

    assert!(
        alive >= 20,
        "resource demo should keep at least 20 cells alive for early UI observation, got {alive}"
    );
}
