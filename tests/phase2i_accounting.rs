use alife::core::accounting::{IntegratedAccountingSnapshot, MatterAccountingDelta};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn accounting_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(8.0, 8.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(50.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(21),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(20.0).unwrap()], 0.0).unwrap(),
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
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn integrated_accounting_snapshot_includes_resources_materials_fragments_and_joints() {
    let executor = TickExecutor::new(accounting_config()).unwrap();

    let snapshot = IntegratedAccountingSnapshot::from_world(executor.world());

    assert!(snapshot.world_resources >= 20.0);
    assert_eq!(snapshot.cell_internal_resources, 4.0);
    assert_eq!(snapshot.cell_materials, 7.0);
    assert_eq!(snapshot.fragment_materials, 0.0);
    assert_eq!(snapshot.joint_materials, 0.0);
    assert!(snapshot.total_matter() >= 31.0);
}

#[test]
fn accounting_delta_accepts_explicit_decay_sink_but_rejects_unclassified_loss() {
    let before = IntegratedAccountingSnapshot {
        world_resources: 10.0,
        cell_internal_resources: 4.0,
        cell_materials: 7.0,
        fragment_materials: 0.0,
        joint_materials: 0.0,
        explicit_sinks: 0.0,
    };
    let after = IntegratedAccountingSnapshot {
        world_resources: 8.0,
        cell_internal_resources: 4.0,
        cell_materials: 7.0,
        fragment_materials: 0.0,
        joint_materials: 0.0,
        explicit_sinks: 2.0,
    };

    let delta = MatterAccountingDelta::between(before, after);

    assert!(delta.is_clean(0.0001));
    assert_eq!(delta.unclassified_loss, 0.0);
}
