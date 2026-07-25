use alife::bootstrap::prepare;
use alife::core::cell_store::CellIndex;
use alife::core::config::{CellInitialConfig, RuntimeConfig};
use alife::core::joints::{JointChannelConfig, JointEndpoints};
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Tick,
};
use alife::core::world::WorldState;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

pub fn build_20k_cells_world(base_config: RuntimeConfig) -> WorldState {
    let mut config = base_config.clone();
    let mut initial_cells = Vec::with_capacity(20_000);

    // Arrange 20,000 cells on a grid (142 x 141 cells, spaced by 6.0)
    let grid_dim = 142;
    let spacing = 6.0;

    for i in 0..20_000 {
        let gx = (i % grid_dim) as f32;
        let gy = (i / grid_dim) as f32;

        let pos = Position::new(20.0 + gx * spacing, 20.0 + gy * spacing);
        let cell_cfg = CellInitialConfig {
            position: pos,
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(10.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(0.1).unwrap(),
            passive_energy_income: EnergyAmount::new(0.0).unwrap(),
            capacity_limit: CapacityAmount::new(20.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(2.0).unwrap(),
            initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
            initial_transport_material: MaterialAmount::new(1.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
            initial_storage_material: MaterialAmount::new(1.0).unwrap(),
            initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
            initial_structural_material: MaterialAmount::new(1.0).unwrap(),
            initial_repair_material: MaterialAmount::new(1.0).unwrap(),
            initial_contractile_material: MaterialAmount::new(0.0).unwrap(),
            initial_sensory_material: MaterialAmount::new(0.0).unwrap(),
        };
        initial_cells.push(cell_cfg);
    }

    config.initial_cells = initial_cells;
    WorldState::from_config(config).expect("20k cells world initialization failed")
}

#[test]
fn test_scale_20k_cells_scenario_parses_and_initializes() {
    let toml_str = include_str!("../config/scenarios/benchmark/scale_20k_cells.toml");
    let doc = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "scale_20k_cells".to_string(),
        content: toml_str.to_string(),
    })
    .expect("scale_20k_cells.toml must parse");
    assert_eq!(doc.id, "scale_20k_cells");

    let prepared = prepare(&doc).expect("scale_20k_cells.toml must prepare");
    let world = build_20k_cells_world(prepared.runtime_config);

    assert_eq!(world.cells().len(), 20_000);
}

#[test]
fn test_scale_40k_joints_scenario_parses_and_initializes() {
    let toml_str = include_str!("../config/scenarios/benchmark/scale_40k_joints.toml");
    let doc = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "scale_40k_joints".to_string(),
        content: toml_str.to_string(),
    })
    .expect("scale_40k_joints.toml must parse");
    assert_eq!(doc.id, "scale_40k_joints");

    let prepared = prepare(&doc).expect("scale_40k_joints.toml must prepare");
    let mut world = build_20k_cells_world(prepared.runtime_config);

    // Attach 40,000 joints connecting neighboring cells
    let grid_dim = 142;
    let mut joints_added = 0;

    for i in 0..20_000 {
        let cell_a = CellIndex::from_raw(i);
        // Connect right neighbor
        if (i + 1) % grid_dim != 0 && (i + 1) < 20_000 {
            let cell_b = CellIndex::from_raw(i + 1);
            if let Some(endpoints) = JointEndpoints::new(cell_a, cell_b) {
                world.joints_mut_for_commit().create(
                    endpoints,
                    MaterialAmount::new(1.0).unwrap(),
                    JointChannelConfig::mechanical_only(1.0),
                    Tick::from_raw(0),
                );
                joints_added += 1;
            }
        }
        // Connect bottom neighbor
        if i + grid_dim < 20_000 {
            let cell_b = CellIndex::from_raw(i + grid_dim);
            if let Some(endpoints) = JointEndpoints::new(cell_a, cell_b) {
                world.joints_mut_for_commit().create(
                    endpoints,
                    MaterialAmount::new(1.0).unwrap(),
                    JointChannelConfig::mechanical_only(1.0),
                    Tick::from_raw(0),
                );
                joints_added += 1;
            }
        }
        if joints_added >= 40_000 {
            break;
        }
    }

    assert_eq!(world.cells().len(), 20_000);
    assert_eq!(world.joints().len(), joints_added);
    assert!(joints_added >= 39_000);
}
