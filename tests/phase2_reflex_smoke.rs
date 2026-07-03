use alife::core::cell_store::CellIndex;
use alife::core::config::RuntimeConfig;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_reflex_config() -> RuntimeConfig {
    use alife::core::config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, SpaceConfig, WorldConfig,
    };

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(100),
            seed: Seed::from_raw(42),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        CellInitialConfig {
            position: Position::new(10.0, 10.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(20.0).unwrap(),
            energy_capacity: EnergyAmount::new(100.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(50.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
            initial_boundary_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_transport_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_storage_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_synthesis_material: MaterialAmount::zero(),
            initial_structural_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_repair_material: MaterialAmount::new(4.0 / 9.0).unwrap(),
            initial_contractile_material: MaterialAmount::zero(),
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
fn test_autonomous_synthesis_and_growth_reflex() {
    let mut config = base_reflex_config();
    config.resource_interaction.enabled = true;
    config.growth_enabled = true;

    // Abundant resources and energy, with synthesis capability enabled
    config.cell.initial_resource_amount = ResourceAmount::new(10.0).unwrap();
    config.cell.initial_synthesis_material = MaterialAmount::new(1.0).unwrap();

    let mut exec = alife::core::tick::TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let old_structural = exec.world().cells().structural_material(idx).raw();
    let old_radius = exec.world().cells().radius(idx).raw();

    // Run one step (executes reflex loop: synthesis then growth)
    exec.step().unwrap();

    // Synthesis should run, increasing structural material
    let new_structural = exec.world().cells().structural_material(idx).raw();
    assert!(new_structural > old_structural);

    // Growth should also run (since growth is enabled and synthesis generated structural material)
    let new_radius = exec.world().cells().radius(idx).raw();
    assert!(new_radius > old_radius);
}

#[test]
fn test_autonomous_displacement_reflex() {
    let mut config = base_reflex_config();
    config.contractility.energy_cost = EnergyAmount::new(1.0).unwrap();
    config.contractility.force_factor = 0.5;

    // Set two overlapping cells with contractility capability enabled
    let mut cell1 = config.cell;
    cell1.position = Position::new(10.0, 10.0);
    cell1.initial_contractile_material = MaterialAmount::new(1.0).unwrap();

    let mut cell2 = cell1;
    cell2.position = Position::new(11.5, 10.0); // overlapping: dx = 1.5, sum_rad = 2.0

    let cells_list = vec![cell1, cell2];
    config = config.with_cells(cells_list);

    let mut exec = alife::core::tick::TickExecutor::new(config).unwrap();
    let idx_1 = CellIndex::from_raw(0);
    let idx_2 = CellIndex::from_raw(1);

    // Manually set pressure > 0 to make contractile displacement feasible
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_contact_pressure(idx_1, 0.5);
        cells.set_contact_pressure(idx_2, 0.5);
    }

    // Run one step (executes reflex loop: contractile displacement runs first)
    exec.step().unwrap();

    // Verify both cells shifted away from each other:
    // Cell 1 should move left (x < 10.0)
    // Cell 2 should move right (x > 11.5)
    assert!(exec.world().cells().position(idx_1).x() < 10.0);
    assert!(exec.world().cells().position(idx_2).x() > 11.5);
}
