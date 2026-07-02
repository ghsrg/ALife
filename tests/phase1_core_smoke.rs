use alife::core;
use alife::core::ids::{CellId, EventId, MaterialTypeId, ResourceTypeId};

#[test]
fn core_module_is_public() {
    let _ = core::CORE_MODULE_NAME;
    assert_eq!(core::CORE_MODULE_NAME, "alife-core");
}

#[test]
fn typed_ids_preserve_raw_values() {
    assert_eq!(CellId::from_raw(7).raw(), 7);
    assert_eq!(ResourceTypeId::from_raw(2).raw(), 2);
    assert_eq!(MaterialTypeId::from_raw(3).raw(), 3);
    assert_eq!(EventId::from_raw(4).raw(), 4);
}

#[test]
fn typed_ids_are_orderable_and_copyable() {
    let a = CellId::from_raw(1);
    let b = a;
    assert_eq!(a, b);
    assert!(CellId::from_raw(1) < CellId::from_raw(2));
}

use alife::core::units::{
    AmountError, CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
    ResourceAmount, Temperature, Tick, WasteAmount, WorldSize,
};

#[test]
fn amount_wrappers_reject_negative_values() {
    assert_eq!(EnergyAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(ResourceAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(MaterialAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(CapacityAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(HeatAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(WasteAmount::new(-0.1), Err(AmountError::Negative));
}

#[test]
fn energy_math_is_saturating_and_clamped() {
    let energy = EnergyAmount::new(2.0).unwrap();
    let cost = EnergyAmount::new(5.0).unwrap();
    let gain = EnergyAmount::new(10.0).unwrap();
    let cap = EnergyAmount::new(6.0).unwrap();

    assert_eq!(energy.saturating_sub(cost).raw(), 0.0);
    assert_eq!(energy.saturating_add(gain).clamp_max(cap).raw(), 6.0);
}

#[test]
fn spatial_wrappers_validate_basic_bounds() {
    assert!(Radius::new(1.0).is_ok());
    assert!(Radius::new(0.0).is_err());
    assert_eq!(Position::new(2.0, 3.0).x(), 2.0);
    assert_eq!(WorldSize::new(512.0, 512.0).unwrap().width(), 512.0);
    assert_eq!(Tick::from_raw(42).raw(), 42);
    assert_eq!(Temperature::new(25.0).raw(), 25.0);
}

use alife::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::Seed;

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

#[test]
fn runtime_config_validates_energy_capacity() {
    let err = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.01).unwrap(),
        ResourceInteractionConfig::disabled(),
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(30.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::new(2.0).unwrap(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
        },
        valid_config().environment,
        valid_config().lifecycle,
    )
    .unwrap_err();

    assert_eq!(err, ConfigError::InitialEnergyExceedsCapacity);
}

use alife::core::world::WorldState;

#[test]
fn world_initializes_one_cell_from_config() {
    let world = WorldState::from_config(valid_config()).unwrap();

    assert_eq!(world.tick().raw(), 0);
    assert_eq!(world.cells().len(), 1);
    assert_eq!(world.environment().heat().raw(), 0.0);
    assert_eq!(world.resources().layer_count(), 1);
}

use alife::core::events::EventKind;
use alife::core::tick::TickExecutor;

#[test]
fn successful_mandatory_cost_does_not_emit_paid_event_spam() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    executor.step().unwrap();

    assert!(
        executor
            .world()
            .events()
            .iter_ordered()
            .all(|event| event.kind != EventKind::MandatoryCostFailed)
    );
    assert_eq!(executor.world().events().len(), 1);
    assert_eq!(
        executor
            .world()
            .events()
            .iter_ordered()
            .next()
            .unwrap()
            .kind,
        EventKind::TickCommitted
    );
}

use alife::core::snapshot::{CommittedSnapshot, ViewerFrame};

#[test]
fn snapshot_and_viewer_frame_are_read_only_projections() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    executor.step().unwrap();

    let snapshot = CommittedSnapshot::from_world(executor.world());
    let frame = ViewerFrame::from_snapshot(&snapshot);

    assert_eq!(snapshot.tick.raw(), 1);
    assert_eq!(snapshot.cells.len(), 1);
    assert_eq!(frame.cells.len(), 1);
    assert_eq!(executor.world().tick().raw(), 1);
}

#[test]
fn snapshot_uses_stored_cell_radius_from_config() {
    let mut config = valid_config();
    config.cell.radius = Radius::new(2.0).unwrap();

    let world = WorldState::from_config(config).unwrap();
    let snapshot = CommittedSnapshot::from_world(&world);

    assert_eq!(snapshot.cells[0].radius.raw(), 2.0);
}
