use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn deterministic_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(50),
            seed: Seed::from_raw(42),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.01).unwrap(),
        ResourceInteractionConfig::disabled(),
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(10.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::new(2.0).unwrap(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
            initial_boundary_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_transport_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_storage_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_synthesis_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_structural_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_repair_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_contractile_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_sensory_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::new(0.1).unwrap(),
            heat_dissipation_rate: HeatAmount::new(0.2).unwrap(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::new(0.1).unwrap(),
            waste_sink_rate: WasteAmount::new(0.2).unwrap(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn same_config_seed_and_binary_produce_same_summary_and_events() {
    let mut first = TickExecutor::new(deterministic_config()).unwrap();
    let mut second = TickExecutor::new(deterministic_config()).unwrap();

    let first_summary = first.run_until_configured_tick().unwrap();
    let second_summary = second.run_until_configured_tick().unwrap();

    let first_events: Vec<_> = first.world().events().iter_ordered().copied().collect();
    let second_events: Vec<_> = second.world().events().iter_ordered().copied().collect();

    assert_eq!(first_summary, second_summary);
    assert_eq!(first_events, second_events);
}
