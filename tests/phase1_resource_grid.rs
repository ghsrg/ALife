use alife::core::config::{ConfigError, ResourceConfig};
use alife::core::units::GridCoord;
use alife::core::units::ResourceAmount;

#[test]
fn grid_coord_preserves_xy_indices() {
    let coord = GridCoord::new(3, 5);

    assert_eq!(coord.x(), 3);
    assert_eq!(coord.y(), 5);
}

#[test]
fn resource_config_rejects_empty_initial_distribution() {
    let err = ResourceConfig::new(Vec::new(), 0.0).unwrap_err();

    assert_eq!(err, ConfigError::EmptyResourceDistribution);
}

#[test]
fn resource_config_rejects_invalid_decay_rate() {
    let err = ResourceConfig::new(vec![ResourceAmount::new(1.0).unwrap()], 1.5).unwrap_err();

    assert_eq!(err, ConfigError::InvalidDecayRate);
}
use alife::core::resources::{ResourceGrid, ResourceGridError, ResourceLayerIndex};
use alife::core::units::WorldSize;

#[test]
fn resource_grid_builds_flat_layers_from_config() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![
            ResourceAmount::new(10.0).unwrap(),
            ResourceAmount::new(5.0).unwrap(),
        ],
        0.1,
    )
    .unwrap();

    assert_eq!(grid.width(), 2);
    assert_eq!(grid.height(), 2);
    assert_eq!(grid.layer_count(), 2);
    assert_eq!(grid.cell_count(), 4);
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(1, 1))
            .unwrap()
            .raw(),
        10.0
    );
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        5.0
    );
}

#[test]
fn resource_grid_rejects_out_of_bounds_access() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![ResourceAmount::new(10.0).unwrap()],
        0.0,
    )
    .unwrap();

    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(2, 0))
            .unwrap_err(),
        ResourceGridError::GridCoordOutOfBounds
    );
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0))
            .unwrap_err(),
        ResourceGridError::LayerOutOfBounds
    );
}

use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceInteractionConfig,
    RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, Seed, Tick,
    WasteAmount,
};
use alife::core::world::WorldState;

fn grid_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(
            vec![
                ResourceAmount::new(10.0).unwrap(),
                ResourceAmount::new(5.0).unwrap(),
            ],
            0.1,
        )
        .unwrap(),
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
fn world_initializes_resource_grid_from_resource_config_not_cell_inventory() {
    let world = WorldState::from_config(grid_config()).unwrap();

    assert_eq!(world.resources().width(), 2);
    assert_eq!(world.resources().height(), 2);
    assert_eq!(world.resources().layer_count(), 2);
    assert_eq!(
        world
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        10.0
    );
    assert_eq!(
        world
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        5.0
    );
}

use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_maps_initial_distribution_to_resource_config_layers() {
    let toml = r#"
scenario_id = "resource_mapping"
seed = 42
tick_count = 10
legacy_material_distribution = true

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["water", "nutrient"]
initial_distribution = [10.0, 5.0]
optional_decay_rate = 0.1
passive_energy_income_placeholder = 2.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = { water = 2.0, nutrient = 1.0 }
initial_materials = { cell_wall = 5.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 30.0
minimum_viability_materials = { cell_wall = 1.0 }

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.1
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_current = 0.0
waste_generated_per_tick = 0.05
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    assert_eq!(config.resources.layer_count(), 2);
    assert_eq!(config.resources.initial_distribution[0].raw(), 10.0);
    assert_eq!(config.resources.initial_distribution[1].raw(), 5.0);
    assert_eq!(config.cell.initial_resource_amount.raw(), 3.0);
}

use alife::core::snapshot::CommittedSnapshot;

#[test]
fn snapshot_contains_resource_layer_totals() {
    let world = WorldState::from_config(grid_config()).unwrap();
    let snapshot = CommittedSnapshot::from_world(&world);

    assert_eq!(snapshot.resource_layer_totals.len(), 2);
    assert_eq!(snapshot.resource_layer_totals[0].raw(), 40.0);
    assert_eq!(snapshot.resource_layer_totals[1].raw(), 20.0);
}
