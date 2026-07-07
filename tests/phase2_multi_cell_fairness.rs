use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn test_config_two_cells() -> RuntimeConfig {
    let cell_a = CellInitialConfig {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(100.0).unwrap(),
        energy_capacity: EnergyAmount::new(200.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        // cell_a starts with 3.0 resources
        initial_resource_amount: ResourceAmount::new(3.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(1.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    };

    let cell_b = CellInitialConfig {
        position: Position::new(10.0, 10.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(100.0).unwrap(),
        energy_capacity: EnergyAmount::new(200.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        // cell_b starts with 7.0 resources
        initial_resource_amount: ResourceAmount::new(7.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(1.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    };

    let cells = vec![cell_a, cell_b];
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(1),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: false, // Disable uptake/metabolism so resource levels stay constant
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::zero(),
            metabolism_resource_per_tick: ResourceAmount::zero(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cells[0],
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
    .with_cells(cells);

    config.synthesis.cost_resource = ResourceAmount::zero();
    config.synthesis.cost_energy = EnergyAmount::zero();
    config
}

#[test]
fn test_two_cell_summary_reports_summed_internal_resources() {
    let config = test_config_two_cells();
    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.step().unwrap();

    println!("SURVIVAL: {:?}", summary.survival_result);
    println!("COLLAPSE REASON: {:?}", summary.collapse_reason);

    // Check that internal resources of both cells are summed: 3.0 + 7.0 = 10.0
    assert_eq!(summary.metrics.final_internal_resources, 10.0);
    // Capacity check: capacity limit 20 each.
    // Cell A has 1.0 structural, 1.0 transport, 1.0 metabolic, 1.0 storage, 1.0 synthesis, 1.0 repair, 1.0 contractile, 1.0 sensory, 1.0 boundary = 9.0 materials
    // Cell A resources = 3.0. Used capacity = 9.0 (materials) + 3.0 (resources) = 12.0. Free capacity = 8.0.
    // Cell B has 9.0 materials + 7.0 resources = 16.0 used capacity. Free capacity = 4.0.
    // Summed used capacity = 13.0 + 17.0 = 30.0.
    // Summed free capacity = 7.0 + 3.0 = 10.0.
    assert_eq!(summary.metrics.final_used_capacity, 30.0);
    assert_eq!(summary.metrics.final_free_capacity, 10.0);
}
