use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};
use alife::runner::config_parser::RawScenarioConfig;

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

#[test]
fn changing_chemistry_registry_property_changes_hash() {
    let config_a = RawScenarioConfig::parse(&chemistry_fixture()).unwrap();
    let config_b = RawScenarioConfig::parse(
        &chemistry_fixture().replace("decay_rate = 0.01", "decay_rate = 0.03"),
    )
    .unwrap();
    assert_ne!(config_a.config_hash(), config_b.config_hash());
}

#[test]
fn changing_reaction_coefficient_changes_hash() {
    let config_a = RawScenarioConfig::parse(&chemistry_fixture()).unwrap();
    let config_b =
        RawScenarioConfig::parse(&chemistry_fixture().replace("rate = 0.2", "rate = 0.3")).unwrap();
    assert_ne!(config_a.config_hash(), config_b.config_hash());
}

fn chemistry_fixture() -> String {
    r#"
scenario_id = "phase2g"
seed = 7
tick_count = 10
[world]
size = [32.0, 32.0]
[space]
spatial_grid_size = 8.0
[resources]
resource_type_ids = ["nutrient_A", "waste_A"]
initial_distribution = [10.0, 0.0]
optional_decay_rate = 0.0
[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 1.0 }
initial_materials = { boundary = 1.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0
[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0
[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 2.0
[chemistry.resources.nutrient_A]
volume = 1.0
diffusion_rate = 0.2
energy_value = 2.0
decay_rate = 0.01
reactivity_profile = "reactive"
permeability = "passive"
[chemistry.resources.waste_A]
volume = 1.0
diffusion_rate = 0.1
energy_value = 0.0
decay_rate = 0.02
reactivity_profile = "stable"
permeability = "blocked"
[chemistry.materials.boundary_polymer_A]
volume = 1.0
stability = 0.8
strength = 0.7
permeability = 0.5
energy_capacity = 0.0
decay_rate = 0.01
repair_resource = "nutrient_A"
repair_amount = 0.25
[chemistry.reactions.passive_decay]
mode = "passive"
inputs = { nutrient_A = 1.0 }
outputs = { waste_A = 1.0 }
configured_sink_amount = 0.0
energy_output = 0.0
heat_output = 0.0
rate = 0.1
probability = 1.0
accounting_destination = "waste_A"
[chemistry.reactions.controlled_conversion]
mode = "controlled"
process_id = "energy_conversion"
inputs = { nutrient_A = 1.0 }
required_materials = { boundary_polymer_A = 0.2 }
outputs = { waste_A = 0.5 }
configured_sink_amount = 0.5
energy_output = 0.8
heat_output = 0.1
rate = 0.2
probability = 0.5
accounting_destination = "waste_A"
[chemistry.heat]
capacity = 10.0
dissipation_rate = 0.2
warning_threshold = 8.0
death_threshold = 10.0
[chemistry.boundary]
default_permeability = "blocked"
retention_rate = 0.9
[chemistry.repair]
enabled = true
energy_cost = 0.5
max_amount_per_tick = 1.0
"#
    .to_string()
}
