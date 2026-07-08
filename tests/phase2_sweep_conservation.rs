use alife::core::cell_store::CellIndex;
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[test]
fn test_conservation_error_calculation() {
    let config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(20),
            seed: Seed::from_raw(42),
            size: WorldSize::new(128.0, 128.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.5).unwrap(),
            energy_per_resource: 10.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.1,
        },
        CellInitialConfig {
            position: Position::new(64.0, 64.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(50.0).unwrap(),
            energy_capacity: EnergyAmount::new(100.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
            initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
            initial_transport_material: MaterialAmount::new(1.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
            initial_storage_material: MaterialAmount::zero(),
            initial_synthesis_material: MaterialAmount::zero(),
            initial_structural_material: MaterialAmount::new(1.0).unwrap(),
            initial_repair_material: MaterialAmount::zero(),
            initial_contractile_material: MaterialAmount::zero(),
            initial_sensory_material: MaterialAmount::zero(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.2).unwrap(),
            heat_warning_threshold: HeatAmount::new(50.0).unwrap(),
            heat_death_threshold: HeatAmount::new(80.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(10.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.1,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let initial_grid = executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .unwrap()
        .raw();
    let initial_cell_res = executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let initial_cell_mat = executor
        .world()
        .cells()
        .total_materials(CellIndex::from_raw(0))
        .raw();
    let initial_total_res = initial_grid + initial_cell_res + initial_cell_mat;

    let mut metabolized_cumulative = 0.0;
    let mut decay_cumulative = 0.0;

    for _ in 0..10 {
        let grid_before = executor
            .world()
            .resources()
            .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
            .unwrap()
            .raw();
        let cell_res_before = executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw();

        let summary = executor.step().unwrap();

        let att = summary
            .diagnostics
            .attempts_by_process
            .get(&alife::core::process::ProcessId::MetabolismEnergyConversion)
            .copied()
            .unwrap_or(0);
        let rej = summary
            .diagnostics
            .rejections_by_process
            .get(&alife::core::process::ProcessId::MetabolismEnergyConversion)
            .copied()
            .unwrap_or(0);
        let metabolism_successes = att.saturating_sub(rej);

        let metabolized_tick = metabolism_successes as f32 * 0.5;
        metabolized_cumulative += metabolized_tick;

        let grid_after = executor
            .world()
            .resources()
            .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
            .unwrap()
            .raw();
        let cell_res_after = executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw();

        let uptake_tick = (cell_res_after - cell_res_before) + metabolized_tick;
        let decay_tick = (grid_before - uptake_tick - grid_after).max(0.0);
        decay_cumulative += decay_tick;

        let final_grid = grid_after;
        let final_cell_res = cell_res_after;
        let final_cell_mat = executor
            .world()
            .cells()
            .total_materials(CellIndex::from_raw(0))
            .raw();

        let current_total_res = final_grid
            + final_cell_res
            + final_cell_mat
            + metabolized_cumulative
            + decay_cumulative;
        assert!(
            (current_total_res - initial_total_res).abs() < 1e-3,
            "Resource conservation error!"
        );
    }
}
