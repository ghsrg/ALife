use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_decomposition_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(32.0, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(80.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(10.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::zero(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };

    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(20),
            seed: Seed::from_raw(7),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(100.0).unwrap(),
            heat_death_threshold: HeatAmount::new(200.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(100.0).unwrap(),
            waste_death_threshold: WasteAmount::new(200.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(5.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(20.0).unwrap(),
        },
    )
    .unwrap();
    config.decomposition.enabled = true;
    config.decomposition.resource_layer_index = 0;
    config.decomposition.resources_per_tick = ResourceAmount::new(5.0).unwrap();
    config.decomposition.materials_per_tick = MaterialAmount::new(1.0).unwrap();
    config
}

#[test]
fn decomposition_converts_dead_cell_resources_and_materials_to_grid_resources_and_marks_inert() {
    let config = base_decomposition_config();
    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    // Set cell lifecycle to Dead
    exec.world_mut()
        .cells_mut_for_commit()
        .set_lifecycle_state(idx, LifecycleState::Dead);

    // Grid resources initially under the cell
    let grid_coord = exec
        .world()
        .resources()
        .coord_for_position(exec.world().cells().position(idx));
    let layer = alife::core::resources::ResourceLayerIndex::from_raw(0);
    assert_eq!(
        exec.world()
            .resources()
            .amount_at(layer, grid_coord)
            .unwrap()
            .raw(),
        0.0
    );

    // Run first tick of decomposition
    let summary = exec.step().unwrap();

    // 5 resources and 1 boundary material must be decomposed.
    // internal resource becomes 5.0, boundary material becomes 0.0.
    // grid resource becomes 5.0 + 1.0 = 6.0
    assert_eq!(exec.world().cells().resource_amount(idx).raw(), 5.0);
    assert_eq!(exec.world().cells().boundary_material(idx).raw(), 0.0);
    assert_eq!(
        exec.world()
            .resources()
            .amount_at(layer, grid_coord)
            .unwrap()
            .raw(),
        6.0
    );
    assert!(!exec.world().cells().runtime_flags(idx).inert);
    assert_eq!(summary.metrics.decomposed_cells_count, 0);

    // Run second tick of decomposition
    let summary_second = exec.step().unwrap();

    // Remaining 5 resources are decomposed
    assert_eq!(exec.world().cells().resource_amount(idx).raw(), 0.0);
    assert_eq!(
        exec.world()
            .resources()
            .amount_at(layer, grid_coord)
            .unwrap()
            .raw(),
        11.0
    );

    // Now resources & materials are fully empty -> runtime flags must have inert = true
    assert!(exec.world().cells().runtime_flags(idx).inert);
    assert_eq!(summary_second.metrics.decomposed_cells_count, 1);
}
