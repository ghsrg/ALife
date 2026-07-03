use alife::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_interaction_config(interaction: ResourceInteractionConfig) -> RuntimeConfig {
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
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        interaction,
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
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
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
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
fn resource_interaction_config_disabled_preserves_default_behavior() {
    let interaction = ResourceInteractionConfig::disabled();

    assert!(!interaction.enabled);
    assert_eq!(interaction.uptake_layer_index, 0);
    assert_eq!(interaction.max_uptake_per_tick.raw(), 0.0);
}

#[test]
fn runtime_config_rejects_enabled_interaction_with_missing_resource_layer() {
    let err = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 1,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.2,
        },
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
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
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
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
    .unwrap_err();

    assert_eq!(err, ConfigError::InvalidResourceInteractionLayer);
}

use alife::core::resources::{ResourceGrid, ResourceLayerIndex};
use alife::core::units::GridCoord;

#[test]
fn resource_grid_maps_position_to_clamped_grid_coord() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![ResourceAmount::new(10.0).unwrap()],
        0.0,
    )
    .unwrap();

    assert_eq!(
        grid.coord_for_position(Position::new(1.0, 1.0)),
        GridCoord::new(0, 0)
    );
    assert_eq!(
        grid.coord_for_position(Position::new(8.0, 1.0)),
        GridCoord::new(1, 0)
    );
    assert_eq!(
        grid.coord_for_position(Position::new(99.0, 99.0)),
        GridCoord::new(1, 1)
    );
}

use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};

fn one_cell_store_with_resources(
    resources: ResourceAmount,
    capacity_limit: CapacityAmount,
) -> CellStore {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(20.0).unwrap(),
        ),
        resources,
        boundary_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        transport_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        metabolic_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        storage_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        synthesis_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        structural_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        repair_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        contractile_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        sensory_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        capacity_limit,
        temperature: alife::core::units::Temperature::new(25.0),
    });
    cells
}

#[test]
fn cell_resource_uptake_is_limited_by_free_capacity() {
    let mut cells = one_cell_store_with_resources(
        ResourceAmount::new(4.0).unwrap(),
        CapacityAmount::new(10.0).unwrap(),
    );

    let accepted = cells.add_resources_limited_by_capacity(
        CellIndex::from_raw(0),
        ResourceAmount::new(10.0).unwrap(),
    );

    assert_eq!(accepted.raw(), 2.0);
    assert_eq!(cells.resource_amount(CellIndex::from_raw(0)).raw(), 6.0);
    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 10.0);
}

#[test]
fn cell_resource_consumption_is_limited_by_available_inventory() {
    let mut cells = one_cell_store_with_resources(
        ResourceAmount::new(3.0).unwrap(),
        CapacityAmount::new(10.0).unwrap(),
    );

    let consumed =
        cells.consume_resources(CellIndex::from_raw(0), ResourceAmount::new(5.0).unwrap());

    assert_eq!(consumed.raw(), 3.0);
    assert_eq!(cells.resource_amount(CellIndex::from_raw(0)).raw(), 0.0);
}

use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;

#[test]
fn tick_uptakes_local_resource_into_cell_inventory() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(3.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::zero(),
        energy_per_resource: 0.0,
        heat_per_resource: 0.0,
        waste_per_resource: 0.0,
    };
    let config = base_interaction_config(interaction);
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(
        executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw(),
        3.0
    );
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        7.0
    );
}

#[test]
fn tick_metabolizes_internal_resource_into_energy_heat_and_waste() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(2.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(2.0).unwrap(),
        energy_per_resource: 3.0,
        heat_per_resource: 0.5,
        waste_per_resource: 0.25,
    };
    let mut config = base_interaction_config(interaction);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(
        executor
            .world()
            .cells()
            .energy(CellIndex::from_raw(0))
            .current()
            .raw(),
        5.0
    );
    assert_eq!(
        executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw(),
        0.0
    );
    assert_eq!(executor.world().environment().heat().raw(), 1.0);
    assert_eq!(executor.world().environment().waste().raw(), 0.5);
}

#[test]
fn cell_survives_from_local_resource_without_passive_income() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config = base_interaction_config(interaction);
    config.world.tick_count = Tick::from_raw(5);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(summary.tick.raw(), 5);
}

#[test]
fn cell_collapses_without_local_resource_or_passive_income() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config = base_interaction_config(interaction);
    config.resources.initial_distribution = vec![ResourceAmount::zero()];
    config.cell.initial_energy = EnergyAmount::new(0.4).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::MandatoryCostUnpaid);
}

use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_maps_resource_interaction_block() {
    let toml = r#"
scenario_id = "resource_interaction"
seed = 42
tick_count = 10

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0
passive_energy_income_placeholder = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.5
heat_per_resource = 0.05
waste_per_resource = 0.05

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { cell_wall = 4.0 }
initial_energy = 1.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.25
capacity_limit = 30.0
minimum_viability_materials = { cell_wall = 1.0 }

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 3.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    assert!(config.resource_interaction.enabled);
    assert_eq!(config.resource_interaction.uptake_layer_index, 0);
    assert_eq!(config.resource_interaction.max_uptake_per_tick.raw(), 1.0);
    assert_eq!(
        config
            .resource_interaction
            .metabolism_resource_per_tick
            .raw(),
        1.0
    );
    assert_eq!(config.resource_interaction.energy_per_resource, 2.5);
}

#[test]
fn resource_interaction_is_deterministic_for_same_config_and_seed() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config_a = base_interaction_config(interaction);
    config_a.world.tick_count = Tick::from_raw(5);
    config_a.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config_a.cell.passive_energy_income = EnergyAmount::zero();

    let config_b = config_a.clone();

    let mut executor_a = TickExecutor::new(config_a).unwrap();
    let mut executor_b = TickExecutor::new(config_b).unwrap();

    let summary_a = executor_a.run_until_configured_tick().unwrap();
    let summary_b = executor_b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a, summary_b);
    assert_eq!(
        executor_a
            .world()
            .resources()
            .quantities()
            .iter()
            .map(|amount| amount.raw())
            .collect::<Vec<_>>(),
        executor_b
            .world()
            .resources()
            .quantities()
            .iter()
            .map(|amount| amount.raw())
            .collect::<Vec<_>>()
    );
}
