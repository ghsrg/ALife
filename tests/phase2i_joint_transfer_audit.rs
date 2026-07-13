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

fn cell_at(x: f32, resources: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(resources).unwrap(),
        initial_boundary_material: MaterialAmount::new(2.0).unwrap(),
        initial_transport_material: MaterialAmount::new(2.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn joint_transfer_config() -> RuntimeConfig {
    let cell = cell_at(30.0, 10.0);
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(71),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
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
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 0.1,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap()
    .with_cells(vec![cell_at(30.0, 10.0), cell_at(33.9, 0.0)]);
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    config.joints.resource_transfer_rate = 0.5;
    config.joints.max_resource_transfer_per_tick = ResourceAmount::new(2.0).unwrap();
    config.local_interaction.enabled = true;
    config
}

#[test]
fn joint_resource_transfer_audit_reports_gross_net_endpoints_and_backflow() {
    let mut exec = TickExecutor::new(joint_transfer_config()).unwrap();
    exec.step().unwrap();

    let summary = exec.step().unwrap();
    let source_final = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let target_final = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    assert!(summary.metrics.joint_resource_transfer_gross_amount > 0.0);
    assert_eq!(
        summary.metrics.joint_resource_transfer_amount,
        summary.metrics.joint_resource_transfer_gross_amount
    );
    assert_eq!(
        summary.metrics.joint_resource_transfer_net_amount,
        summary.metrics.joint_resource_transfer_gross_amount
    );
    assert_eq!(
        summary.metrics.joint_resource_source_final_amount,
        source_final
    );
    assert_eq!(
        summary.metrics.joint_resource_target_final_amount,
        target_final
    );
    assert_eq!(summary.metrics.joint_resource_backflow_amount, 0.0);
}
