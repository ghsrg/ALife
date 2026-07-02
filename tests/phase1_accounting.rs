use alife::core::cell_store::{
    CellIndex, CellStore, EnergyBuffer, InitialCellState, LifecycleState,
};
use alife::core::ids::CellId;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[test]
fn cell_store_inserts_one_cell_with_deterministic_id() {
    let mut cells = CellStore::with_capacity(1);
    let id = cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 2.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::new(4.0).unwrap(),
        materials: MaterialAmount::new(3.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    assert_eq!(id, CellId::from_raw(1));
    assert_eq!(cells.len(), 1);
    assert_eq!(cells.id_at(CellIndex::from_raw(0)), id);
    assert_eq!(cells.position(CellIndex::from_raw(0)).x(), 1.0);
}

#[test]
fn capacity_accounting_excludes_energy_and_includes_resources_materials() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 2.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(100.0).unwrap(),
            EnergyAmount::new(100.0).unwrap(),
        ),
        resources: ResourceAmount::new(4.0).unwrap(),
        materials: MaterialAmount::new(3.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 7.0);
    assert_eq!(cells.free_capacity(CellIndex::from_raw(0)).raw(), 3.0);
}

use alife::core::lifecycle::{LifecycleInput, LifecycleReason, evaluate_lifecycle};

#[test]
fn lifecycle_priority_prefers_death_over_dormancy_and_stress() {
    let decision = evaluate_lifecycle(LifecycleInput {
        mandatory_paid: false,
        energy_after_mandatory: EnergyAmount::zero(),
        stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
        over_capacity: true,
        critical_capacity_exceeded: true,
        heat_warning: true,
        heat_death: true,
        waste_warning: true,
        waste_death: true,
        dormancy_allowed: true,
        dormant_cost_payable: true,
    });

    assert_eq!(decision.state, LifecycleState::Dead);
    assert_eq!(decision.reason, LifecycleReason::EnergyDepleted);
}

#[test]
fn lifecycle_enters_dormancy_when_unpaid_but_dormant_cost_is_payable() {
    let decision = evaluate_lifecycle(LifecycleInput {
        mandatory_paid: false,
        energy_after_mandatory: EnergyAmount::new(1.0).unwrap(),
        stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
        over_capacity: false,
        critical_capacity_exceeded: false,
        heat_warning: false,
        heat_death: false,
        waste_warning: false,
        waste_death: false,
        dormancy_allowed: true,
        dormant_cost_payable: true,
    });

    assert_eq!(decision.state, LifecycleState::Dormant);
    assert_eq!(decision.reason, LifecycleReason::Dormancy);
}

use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig, RuntimeConfig,
    SpaceConfig, WorldConfig,
};
use alife::core::resources::ResourceLayerIndex;
use alife::core::units::GridCoord;
use alife::core::units::{HeatAmount, Seed, Tick, WasteAmount, WorldSize};

pub fn valid_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.01).unwrap(),
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(10.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::new(2.0).unwrap(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
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

use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;

#[test]
fn tick_executor_pays_mandatory_cost_and_advances_tick() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    let summary = executor.step().unwrap();
    let cell = executor.world().cells().energy(CellIndex::from_raw(0));

    assert_eq!(executor.world().tick().raw(), 1);
    assert_eq!(cell.current().raw(), 10.0);
    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
}

#[test]
fn tick_executor_collapses_on_energy_depletion() {
    let mut config = valid_config();
    config.cell.initial_energy = EnergyAmount::new(0.5).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::EnergyDepleted);
}

#[test]
fn tick_executor_reports_heat_limit_collapse() {
    let mut config = valid_config();
    config.environment.heat_current = HeatAmount::new(19.5).unwrap();
    config.environment.heat_generated_per_tick = HeatAmount::new(2.0).unwrap();
    config.environment.heat_dissipation_rate = HeatAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::HeatLimitExceeded);
}

#[test]
fn tick_executor_reports_waste_limit_collapse() {
    let mut config = valid_config();
    config.environment.waste_current = WasteAmount::new(19.5).unwrap();
    config.environment.waste_generated_per_tick = WasteAmount::new(2.0).unwrap();
    config.environment.waste_sink_rate = WasteAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::WasteLimitExceeded);
}

#[test]
fn tick_executor_decays_resource_grid() {
    let mut config = valid_config();
    config.resources.optional_decay_rate = 0.1;
    config.resources.initial_distribution = vec![ResourceAmount::new(10.0).unwrap()];

    let mut executor = TickExecutor::new(config).unwrap();
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        10.0
    );

    let _ = executor.step().unwrap();
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        9.0
    );

    let _ = executor.step().unwrap();
    let val2 = executor
        .world()
        .resources()
        .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
        .unwrap()
        .raw();
    assert!(
        (val2 - 8.1).abs() < 1e-5,
        "Expected approx 8.1, got {}",
        val2
    );
}
