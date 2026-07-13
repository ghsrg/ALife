use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};
use alife::core::config::{
    CellInitialConfig, ChemistryConfig, ChemistryRepairConfig, EnvironmentConfig, LifecycleConfig,
    ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::heat::LocalHeat;
use alife::core::materials::MaterialSlot;
use alife::core::resource_types::PermeabilityConstraint;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Temperature, Tick, WasteAmount, WorldSize,
};

fn cell() -> InitialCellState {
    InitialCellState {
        position: Position::new(0.0, 0.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::zero(), EnergyAmount::new(1.0).unwrap()),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::zero(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::zero(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    }
}

#[test]
fn reaction_heat_changes_temperature_by_heat_capacity() {
    let mut heat = LocalHeat::new(Temperature::new(20.0), HeatAmount::new(2.0).unwrap());

    heat.add_generated(HeatAmount::new(1.0).unwrap());
    heat.commit();

    assert_eq!(heat.temperature(), Temperature::new(20.5));
}

#[test]
fn local_heat_dissipates_toward_ambient_at_configured_rate() {
    let mut heat = LocalHeat::new(Temperature::new(30.0), HeatAmount::new(2.0).unwrap());

    heat.dissipate_toward(Temperature::new(20.0), 0.25);

    assert_eq!(heat.temperature(), Temperature::new(27.5));
}

#[test]
fn material_over_tolerance_degrades_without_hp_shortcut() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(cell());
    let index = CellIndex::from_raw(0);

    cells.apply_thermal_damage(
        index,
        MaterialSlot::Boundary,
        Temperature::new(35.0),
        30.0,
        0.5,
    );

    assert_eq!(cells.material_damage(index, MaterialSlot::Boundary), 0.5);
    assert_eq!(
        cells.boundary_material(index),
        MaterialAmount::new(0.5).unwrap()
    );
}

#[test]
fn boundary_default_is_blocked_without_compatible_material() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(cell());
    let index = CellIndex::from_raw(0);

    assert!(!cells.boundary_allows_passive_exchange(index));
}

#[test]
fn boundary_damage_increases_only_physically_compatible_leakage() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(cell());
    let index = CellIndex::from_raw(0);

    cells.set_transport_material(index, MaterialAmount::new(1.0).unwrap());
    cells.set_material_damage(index, MaterialSlot::Boundary, 0.5);

    assert_eq!(
        cells.boundary_leakage_rate(index, PermeabilityConstraint::Passive, 0.2),
        0.1
    );
    assert_eq!(
        cells.boundary_leakage_rate(index, PermeabilityConstraint::Blocked, 0.2),
        0.0
    );
}

#[test]
fn missing_boundary_rule_does_not_allow_resource_exchange() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(cell());
    let index = CellIndex::from_raw(0);

    cells.set_transport_material(index, MaterialAmount::new(1.0).unwrap());
    cells.set_material_damage(index, MaterialSlot::Boundary, 0.5);

    assert_eq!(
        cells.boundary_leakage_rate(index, PermeabilityConstraint::ActiveRequired, 0.2),
        0.0
    );
}

fn repair_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(5.0).unwrap(),
        energy_capacity: EnergyAmount::new(10.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(1.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(0.5).unwrap(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::zero(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(9),
            size: WorldSize::new(8.0, 8.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 0,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
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
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap();
    config.chemistry = ChemistryConfig {
        repair: ChemistryRepairConfig {
            enabled: true,
            energy_cost: 1.0,
            max_amount_per_tick: 0.25,
        },
        ..ChemistryConfig::default()
    };
    config
}

#[test]
fn repair_consumes_declared_resource_material_and_energy_inputs() {
    let mut executor = TickExecutor::new(repair_config()).unwrap();
    let index = CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(index, MaterialSlot::Boundary, 0.5);

    executor.step().unwrap();

    let cells = executor.world().cells();
    assert_eq!(
        cells.resource_amount(index),
        ResourceAmount::new(0.75).unwrap()
    );
    assert_eq!(
        cells.repair_material(index),
        MaterialAmount::new(0.75).unwrap()
    );
    assert_eq!(
        cells.energy(index).current(),
        EnergyAmount::new(4.0).unwrap()
    );
}

#[test]
fn repair_reduces_material_damage_when_feasibility_passes() {
    let mut executor = TickExecutor::new(repair_config()).unwrap();
    let index = CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(index, MaterialSlot::Boundary, 0.5);

    let summary = executor.step().unwrap();

    let cells = executor.world().cells();
    assert!((cells.material_damage(index, MaterialSlot::Boundary) - 0.25).abs() < 1e-6);
    assert_eq!(
        cells.boundary_material(index),
        MaterialAmount::new(0.75).unwrap()
    );
    assert_eq!(summary.metrics.repair_success_count, 1);
    assert_eq!(summary.metrics.repair_rejection_count, 0);
}

#[test]
fn repair_rejects_without_capability_or_required_inputs() {
    let mut config = repair_config();
    config.cell.initial_repair_material = MaterialAmount::zero();
    let mut executor = TickExecutor::new(config).unwrap();
    let index = CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(index, MaterialSlot::Boundary, 0.5);

    let summary = executor.step().unwrap();

    assert_eq!(summary.metrics.repair_success_count, 0);
    assert_eq!(summary.metrics.repair_rejection_count, 1);
}

#[test]
fn rejected_repair_has_no_partial_consumption() {
    let mut config = repair_config();
    config.cell.initial_energy = EnergyAmount::zero();
    let mut executor = TickExecutor::new(config).unwrap();
    let index = CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(index, MaterialSlot::Boundary, 0.5);

    executor.step().unwrap();

    let cells = executor.world().cells();
    assert_eq!(
        cells.resource_amount(index),
        ResourceAmount::new(1.0).unwrap()
    );
    assert_eq!(
        cells.repair_material(index),
        MaterialAmount::new(1.0).unwrap()
    );
    assert_eq!(
        cells.boundary_material(index),
        MaterialAmount::new(0.5).unwrap()
    );
}

#[test]
fn repair_cannot_restore_material_above_configured_amount_or_capacity() {
    let mut executor = TickExecutor::new(repair_config()).unwrap();
    let index = CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(index, MaterialSlot::Boundary, 0.1);

    executor.step().unwrap();

    let cells = executor.world().cells();
    assert!((cells.material_damage(index, MaterialSlot::Boundary)).abs() < 1e-6);
    assert_eq!(
        cells.boundary_material(index),
        MaterialAmount::new(0.6).unwrap()
    );
}
