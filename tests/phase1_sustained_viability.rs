use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

fn one_cell_store() -> CellStore {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(8.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::new(3.0).unwrap(),
        boundary_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        transport_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        metabolic_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        storage_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        synthesis_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        structural_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        repair_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        contractile_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        sensory_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
        capacity_limit: CapacityAmount::new(12.0).unwrap(),
        temperature: Temperature::new(25.0),
    });
    cells
}

#[test]
fn cell_store_exposes_capacity_limit_for_observer_summary() {
    let cells = one_cell_store();

    assert_eq!(cells.capacity_limit(CellIndex::from_raw(0)).raw(), 12.0);
    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 7.0);
    assert_eq!(cells.free_capacity(CellIndex::from_raw(0)).raw(), 5.0);
}

use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;
use alife::core::units::{HeatAmount, Seed, Tick, WasteAmount, WorldSize};

fn viability_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
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
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 3.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(10.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(20.0).unwrap(),
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
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn run_summary_reports_resource_capacity_and_growth_readiness_metrics() {
    let mut config = viability_config();
    config.growth.growth_target_radius = Radius::new(1.0).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert!(summary.metrics.max_energy >= summary.metrics.min_energy);
    assert_eq!(summary.metrics.final_internal_resources, 0.0);
    assert!(summary.metrics.final_external_resources > 0.0);
    assert_eq!(summary.metrics.final_used_capacity, 4.0);
    assert_eq!(summary.metrics.final_free_capacity, 16.0);
    assert!(summary.metrics.growth_readiness);
}

#[test]
fn run_until_configured_tick_tracks_energy_range_across_ticks() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(4);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.metrics.min_energy, 2.0);
    assert_eq!(summary.metrics.max_energy, 5.0);
}

#[test]
fn cell_remains_stable_for_1000_ticks_on_local_resource_loop() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(1_000);
    config.resources.initial_distribution = vec![ResourceAmount::new(2_000.0).unwrap()];
    config.cell.initial_energy = EnergyAmount::new(5.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(20.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 2.2;
    config.resource_interaction.heat_per_resource = 0.01;
    config.resource_interaction.waste_per_resource = 0.01;
    config.environment.heat_dissipation_rate = HeatAmount::new(0.02).unwrap();
    config.environment.waste_sink_rate = WasteAmount::new(0.02).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.tick.raw(), 1_000);
    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert!(summary.metrics.min_energy > 0.0);
    assert!(summary.metrics.heat < 10.0);
    assert!(summary.metrics.waste < 10.0);
}

#[test]
fn cell_collapses_when_local_resources_are_exhausted() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(20);
    config.world.size = WorldSize::new(8.0, 8.0).unwrap();
    config.space.spatial_grid_size = 8.0;
    config.resources.initial_distribution = vec![ResourceAmount::new(1.5).unwrap()];
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 2.5;
    config.lifecycle.dormancy_allowed = false;

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::MandatoryCostUnpaid);
    assert_eq!(summary.metrics.final_external_resources, 0.0);
}

#[test]
fn sustained_metabolism_collapses_when_heat_has_no_sufficient_sink() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(50);
    config.resources.initial_distribution = vec![ResourceAmount::new(100.0).unwrap()];
    config.resource_interaction.heat_per_resource = 1.0;
    config.resource_interaction.waste_per_resource = 0.0;
    config.environment.heat_dissipation_rate = HeatAmount::zero();
    config.environment.heat_warning_threshold = HeatAmount::new(5.0).unwrap();
    config.environment.heat_death_threshold = HeatAmount::new(8.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::HeatLimitExceeded);
    assert!(summary.tick.raw() < 50);
}

#[test]
fn sustained_metabolism_collapses_when_waste_has_no_sufficient_sink() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(50);
    config.resources.initial_distribution = vec![ResourceAmount::new(100.0).unwrap()];
    config.resource_interaction.heat_per_resource = 0.0;
    config.resource_interaction.waste_per_resource = 1.0;
    config.environment.waste_sink_rate = WasteAmount::zero();
    config.environment.waste_warning_threshold = WasteAmount::new(5.0).unwrap();
    config.environment.waste_death_threshold = WasteAmount::new(8.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::WasteLimitExceeded);
    assert!(summary.tick.raw() < 50);
}
