use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_test_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(100),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
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
fn runtime_config_supports_multi_cell_list() {
    let base = base_test_config();
    assert_eq!(base.initial_cells.len(), 1);

    let cell_1 = base.cell;
    let cell_2 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        ..cell_1
    };
    let multi = base.with_cells(vec![cell_1, cell_2]);
    assert_eq!(multi.initial_cells.len(), 2);
}

#[test]
fn parser_loads_multiple_initial_cells() {
    use alife::runner::config_parser::RawScenarioConfig;

    let toml = r#"
scenario_id = "multi_cell_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = {}
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[[cells]]
initial_position = [2.0, 2.0]
radius = 1.0
initial_resources = {}
initial_materials = {}
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[[cells]]
initial_position = [3.0, 3.0]
radius = 1.5
initial_resources = {}
initial_materials = {}
initial_energy = 8.0
energy_capacity = 12.0
mandatory_cost_per_tick = 2.0
capacity_limit = 25.0

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
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.initial_cells.len(), 2); // [[cells]] blocks parse.
    assert_eq!(config.initial_cells[0].position.x(), 2.0);
    assert_eq!(config.initial_cells[1].position.x(), 3.0);
}

#[test]
fn world_state_initializes_multiple_cells_from_config() {
    use alife::core::cell_store::CellIndex;
    use alife::core::world::WorldState;

    let base = base_test_config();
    let cell_1 = base.cell;
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 5.0),
        ..cell_1
    };
    let config = base.with_cells(vec![cell_1, cell_2]);

    let world = WorldState::from_config(config).unwrap();
    assert_eq!(world.cells().len(), 2);
    assert_eq!(world.cells().position(CellIndex::from_raw(0)).x(), 1.0);
    assert_eq!(world.cells().position(CellIndex::from_raw(1)).x(), 5.0);
}

#[test]
fn spatial_index_rebuilds_and_queries_neighbors() {
    use alife::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
    use alife::core::spatial::SpatialIndex;
    use alife::core::units::Temperature;

    let mut cells = CellStore::with_capacity(3);
    // Cell 0 at (1.0, 1.0)
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
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
    });
    // Cell 1 at (2.0, 1.0) -> same grid cell or neighbor
    cells.insert_initial(InitialCellState {
        position: Position::new(2.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
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
    });
    // Cell 2 at (25.0, 25.0) -> far away
    cells.insert_initial(InitialCellState {
        position: Position::new(25.0, 25.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
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
    });

    let mut spatial = SpatialIndex::new();
    spatial.rebuild(&cells, WorldSize::new(32.0, 32.0).unwrap(), 8.0);

    let mut pairs = Vec::new();
    spatial.generate_candidate_pairs(&cells, &mut pairs);

    // Expect pairs (0, 1), not (0, 2) or (1, 2)
    assert_eq!(pairs.len(), 1);
    let pair = pairs[0];
    assert_eq!(pair.0.raw(), 0);
    assert_eq!(pair.1.raw(), 1);
}

#[test]
fn parser_loads_solver_iterations_from_space_config() {
    use alife::runner::config_parser::RawScenarioConfig;

    let toml = r#"
scenario_id = "solver_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 8

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = {}
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

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
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.space.physics_solver_iterations, 8);
}

#[test]
fn tick_executor_resolves_overlaps_and_clamped_by_walls() {
    use alife::core::cell_store::CellIndex;
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;

    let base = base_test_config();
    let cell_0 = base.cell;
    // Cell 0 at (4.0, 4.0), radius 2.0
    let cell_1 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..cell_0
    };
    // Cell 1 at (5.0, 4.0), radius 2.0 -> Overlap distance = 3.0
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..cell_0
    };
    let config = base.with_cells(vec![cell_1, cell_2]);

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    let p1 = executor.world().cells().position(CellIndex::from_raw(0));
    let p2 = executor.world().cells().position(CellIndex::from_raw(1));

    // Cells should separate along the X axis
    assert!(p1.x() < 4.0);
    assert!(p2.x() > 5.0);
    // Distance should be close to 4.0 (combined radii)
    let dist = ((p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2)).sqrt();
    assert!(dist >= 3.9);
}

#[test]
fn multi_cell_world_retains_perfect_determinism_and_solid_walls() {
    use alife::core::cell_store::CellIndex;
    use alife::core::tick::TickExecutor;

    // 1. Determinism check
    let base = base_test_config();
    let cell_0 = base.cell;
    let cell_1 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..cell_0
    };
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..cell_0
    };
    let config1 = base.with_cells(vec![cell_1, cell_2]);
    let config2 = config1.clone();

    let mut exec1 = TickExecutor::new(config1).unwrap();
    let mut exec2 = TickExecutor::new(config2).unwrap();

    for _ in 0..10 {
        let _ = exec1.step().unwrap();
        let _ = exec2.step().unwrap();
    }

    let p1_a = exec1.world().cells().position(CellIndex::from_raw(0));
    let p1_b = exec1.world().cells().position(CellIndex::from_raw(1));
    let p2_a = exec2.world().cells().position(CellIndex::from_raw(0));
    let p2_b = exec2.world().cells().position(CellIndex::from_raw(1));

    assert_eq!(p1_a.x(), p2_a.x());
    assert_eq!(p1_a.y(), p2_a.y());
    assert_eq!(p1_b.x(), p2_b.x());
    assert_eq!(p1_b.y(), p2_b.y());

    // 2. Wall Boundary overlap resolution (position (0.5, 0.5) with radius 1.5 -> clamped to (1.5, 1.5))
    let base_wall = base_test_config();
    let cell_wall = CellInitialConfig {
        position: Position::new(0.5, 0.5),
        radius: Radius::new(1.5).unwrap(),
        ..base_wall.cell
    };
    let config_wall = base_wall.with_cells(vec![cell_wall]);
    let mut exec_wall = TickExecutor::new(config_wall).unwrap();
    let _ = exec_wall.step().unwrap();

    let pos_wall = exec_wall.world().cells().position(CellIndex::from_raw(0));
    assert_eq!(pos_wall.x(), 1.5);
    assert_eq!(pos_wall.y(), 1.5);
}
