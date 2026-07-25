use alife::core::cell_store::CellIndex;
use alife::core::config::RuntimeConfig;
use alife::core::joints::{JointChannelConfig, JointEndpoints};
use alife::core::tick::TickExecutor;
use alife::core::units::{MaterialAmount, Tick};
use alife::observer::organism_view::build_organism_view_projection;
use alife::observer::projection_envelope::ProjectionCompleteness;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use std::path::PathBuf;

fn get_base_runtime_config() -> RuntimeConfig {
    let source = ScenarioSource::Path(PathBuf::from(
        "config/scenarios/bootstrap/diverse_rich_world.toml",
    ));
    let document = ScenarioDocument::resolve(source).unwrap();
    document.runtime_config
}

#[test]
fn test_organism_view_single_unattached_cells() {
    let config = get_base_runtime_config();
    let executor = TickExecutor::new(config).unwrap();
    let world = executor.world();

    let projection = build_organism_view_projection(world);

    assert_eq!(projection.tick, 0);
    assert_eq!(projection.total_organisms_count, 5);
    assert_eq!(projection.unattached_cells_count, 5);
    assert_eq!(projection.completeness, ProjectionCompleteness::full());

    for (i, organism) in projection.organisms.iter().enumerate() {
        assert_eq!(organism.total_cells_count, 1);
        assert_eq!(organism.cell_ids, vec![i as u32]);
        assert_eq!(organism.primary_cell_id, i as u32);
        assert_eq!(organism.total_joints_count, 0);
        assert!(organism.total_mass > 0.0);
        assert!(organism.total_energy > 0.0);
    }
}

#[test]
fn test_organism_view_multicellular_connected_component() {
    let config = get_base_runtime_config();
    let mut executor = TickExecutor::new(config).unwrap();

    // Create an active Joint between Cell 0 and Cell 1
    let cell0 = CellIndex::from_raw(0);
    let cell1 = CellIndex::from_raw(1);
    let endpoints = JointEndpoints::new(cell0, cell1).unwrap();
    let channel_config = JointChannelConfig::mechanical_only(1.0);
    let material_cost = MaterialAmount::new(1.0).unwrap();

    executor.world_mut().joints_mut_for_commit().create(
        endpoints,
        material_cost,
        channel_config,
        Tick::from_raw(0),
    );

    let projection = build_organism_view_projection(executor.world());

    // Originally 5 isolated cells. Cell 0 + Cell 1 merged -> 4 total organisms (1 multicellular, 3 unattached)
    assert_eq!(projection.total_organisms_count, 4);
    assert_eq!(projection.unattached_cells_count, 3);

    // Multicellular organism (Cell 0 + Cell 1)
    let multicellular = &projection.organisms[0];
    assert_eq!(multicellular.total_cells_count, 2);
    assert_eq!(multicellular.cell_ids, vec![0, 1]);
    assert_eq!(multicellular.primary_cell_id, 0);
    assert_eq!(multicellular.total_joints_count, 1);
    assert!(multicellular.total_mass > 0.0);
    assert!(multicellular.total_energy > 0.0);

    // Unattached single-cell organisms (Cell 2, Cell 3, Cell 4)
    for organism in &projection.organisms[1..] {
        assert_eq!(organism.total_cells_count, 1);
        assert_eq!(organism.total_joints_count, 0);
    }
}

#[test]
fn test_organism_view_read_only_boundary() {
    let config = get_base_runtime_config();
    let executor = TickExecutor::new(config).unwrap();

    let initial_tick = executor.world().tick();
    let initial_cell_count = executor.world().cells().len();

    let _projection = build_organism_view_projection(executor.world());

    // Verify Read-Only Observer Boundary
    assert_eq!(executor.world().tick(), initial_tick);
    assert_eq!(executor.world().cells().len(), initial_cell_count);
}
