use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
    ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
};

fn base_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(5.0).unwrap(),
        energy_capacity: EnergyAmount::new(10.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
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

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::zero(),
            metabolism_resource_per_tick: ResourceAmount::zero(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
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
}

#[test]
fn changing_world_size_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.world.size = WorldSize::new(32.0, 32.0).unwrap();
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_spatial_grid_size_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.space.spatial_grid_size = 16.0;
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_physics_iterations_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.space.physics_solver_iterations = 8;
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_cell_position_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.cell.position = Position::new(2.0, 2.0);
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_cell_radius_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.cell.radius = Radius::new(2.0).unwrap();
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_lifecycle_threshold_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.lifecycle.stress_energy_threshold = EnergyAmount::new(3.0).unwrap();
    assert_ne!(config1.config_hash(), config2.config_hash());
}

#[test]
fn changing_growth_enabled_changes_hash() {
    let config1 = base_config();
    let mut config2 = base_config();
    config2.growth_enabled = true;
    assert_ne!(config1.config_hash(), config2.config_hash());
}
