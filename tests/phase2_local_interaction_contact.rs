use alife::core::{
    cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState},
    config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
    },
    contact::ContactCache,
    spatial::SpatialIndex,
    tick::TickExecutor,
    units::{
        CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
        Seed, Tick, WasteAmount, WorldSize,
    },
};

fn cell_at(x: f32, y: f32, radius: f32) -> InitialCellState {
    InitialCellState {
        position: Position::new(x, y),
        radius: Radius::new(radius).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(10.0).unwrap(),
            EnergyAmount::new(20.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(1.0).unwrap(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::new(1.0).unwrap(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        temperature: alife::core::units::Temperature::new(25.0),
    }
}

fn config_cell_at(x: f32, y: f32, radius: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, y),
        radius: Radius::new(radius).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn two_overlapping_cells_config() -> RuntimeConfig {
    let first = config_cell_at(10.0, 10.0, 2.0);
    let second = config_cell_at(13.0, 10.0, 2.0);
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(1),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        first,
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
    .with_cells(vec![first, second])
}

#[test]
fn contact_cache_records_only_overlapping_pairs_in_stable_order() {
    let mut cells = CellStore::with_capacity(4);
    cells.insert_initial(cell_at(10.0, 10.0, 2.0));
    cells.insert_initial(cell_at(13.0, 10.0, 2.0));
    cells.insert_initial(cell_at(16.0, 10.0, 2.0));
    cells.insert_initial(cell_at(40.0, 40.0, 1.0));

    let mut spatial = SpatialIndex::new();
    spatial.rebuild(&cells, WorldSize::new(64.0, 64.0).unwrap(), 8.0);

    let mut cache = ContactCache::default();
    cache.rebuild(&cells, &spatial);

    let pairs: Vec<_> = cache
        .pairs()
        .iter()
        .map(|pair| (pair.a.raw(), pair.b.raw(), pair.overlap))
        .collect();

    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, CellIndex::from_raw(0).raw());
    assert_eq!(pairs[0].1, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].0, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].1, CellIndex::from_raw(2).raw());
    assert!(pairs[0].2 > 0.9 && pairs[0].2 < 1.1);
    assert!(cache.total_overlap() > 1.9 && cache.total_overlap() < 2.1);
    assert!(cache.max_overlap() > 0.9 && cache.max_overlap() < 1.1);
}

#[test]
fn run_summary_reports_contact_pairs_and_pressure() {
    let mut exec = TickExecutor::new(two_overlapping_cells_config()).unwrap();
    let summary = exec.step().unwrap();

    assert_eq!(summary.metrics.contact_pairs_count, 1);
    assert!(summary.metrics.contact_pressure_pre_total > 0.0);
    assert!(summary.metrics.contact_pressure_max_over_tick > 0.0);
}
