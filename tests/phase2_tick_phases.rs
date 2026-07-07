// Task 6 — tick phase order: contact sensing BEFORE material reflex loop

fn two_cell_config_with_overlap() -> alife::core::config::RuntimeConfig {
    use alife::core::{
        config::{
            CellInitialConfig, EnvironmentConfig, GrowthConfig, LifecycleConfig, ResourceConfig,
            ResourceInteractionConfig, SpaceConfig, WorldConfig,
        },
        units::{
            CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
            ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
        },
    };
    let make_cell = |x: f32| CellInitialConfig {
        position: Position::new(x, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(0.0).unwrap(),
        initial_transport_material: MaterialAmount::new(0.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(0.0).unwrap(),
        initial_storage_material: MaterialAmount::new(0.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(0.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(0.0).unwrap(),
        // contractile_material present so displacement CAN fire
        initial_contractile_material: MaterialAmount::new(5.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(0.0).unwrap(),
    };
    // Cell A at (32,32), Cell B at (33,32): both r=2, so overlap = (2+2) - 1 = 3
    let cells = vec![make_cell(32.0), make_cell(33.0)];
    let mut config = alife::core::config::RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(42),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(0.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(0.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.0).unwrap(),
            energy_per_resource: 1.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cells[0],
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.0).unwrap(),
            heat_warning_threshold: HeatAmount::new(80.0).unwrap(),
            heat_death_threshold: HeatAmount::new(100.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.0).unwrap(),
            waste_warning_threshold: WasteAmount::new(80.0).unwrap(),
            waste_death_threshold: WasteAmount::new(100.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap()
    .with_cells(cells);
    config.growth_enabled = false;
    config.contractility.energy_cost = EnergyAmount::new(0.0).unwrap();
    config.contractility.force_factor = 1.0;
    config.growth = GrowthConfig {
        growth_cost_resource: ResourceAmount::new(999.0).unwrap(),
        growth_cost_energy: EnergyAmount::new(999.0).unwrap(),
        growth_target_radius: Radius::new(10.0).unwrap(),
        max_division_pressure: 0.1,
    };
    config
}

#[test]
fn test_overlapping_contractile_cells_move_without_manual_pressure_injection() {
    use alife::core::{cell_store::CellIndex, tick::TickExecutor};

    let config = two_cell_config_with_overlap();
    let mut exec = TickExecutor::new(config).unwrap();

    let pos_a_before = exec.world().cells().position(CellIndex::from_raw(0));

    // Run 2 ticks WITHOUT any manual set_contact_pressure calls
    exec.step().unwrap();
    exec.step().unwrap();

    let pos_a_after = exec.world().cells().position(CellIndex::from_raw(0));

    // At least one cell must have moved due to automatic contact sensing + displacement
    let moved = (pos_a_after.x() - pos_a_before.x()).abs() > 0.001
        || (pos_a_after.y() - pos_a_before.y()).abs() > 0.001;

    assert!(
        moved,
        "Cell A must have moved after 2 ticks with overlapping neighbor: before=({}, {}) after=({}, {})",
        pos_a_before.x(),
        pos_a_before.y(),
        pos_a_after.x(),
        pos_a_after.y()
    );
}

fn two_cell_config_no_overlap() -> alife::core::config::RuntimeConfig {
    use alife::core::{
        config::{
            CellInitialConfig, EnvironmentConfig, GrowthConfig, LifecycleConfig, ResourceConfig,
            ResourceInteractionConfig, SpaceConfig, WorldConfig,
        },
        units::{
            CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
            ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
        },
    };
    let make_cell = |x: f32| CellInitialConfig {
        position: Position::new(x, 32.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(0.0).unwrap(),
        initial_transport_material: MaterialAmount::new(0.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(0.0).unwrap(),
        initial_storage_material: MaterialAmount::new(0.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(0.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(0.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(5.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(0.0).unwrap(),
    };
    // 40 units apart, radius=1: no overlap
    let cells = vec![make_cell(10.0), make_cell(50.0)];
    let mut config = alife::core::config::RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(42),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(0.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(0.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.0).unwrap(),
            energy_per_resource: 1.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cells[0],
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.0).unwrap(),
            heat_warning_threshold: HeatAmount::new(80.0).unwrap(),
            heat_death_threshold: HeatAmount::new(100.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.0).unwrap(),
            waste_warning_threshold: WasteAmount::new(80.0).unwrap(),
            waste_death_threshold: WasteAmount::new(100.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap()
    .with_cells(cells);
    config.growth_enabled = false;
    config.contractility.energy_cost = EnergyAmount::new(0.0).unwrap();
    config.contractility.force_factor = 1.0;
    config.growth = GrowthConfig {
        growth_cost_resource: ResourceAmount::new(999.0).unwrap(),
        growth_cost_energy: EnergyAmount::new(999.0).unwrap(),
        growth_target_radius: Radius::new(10.0).unwrap(),
        max_division_pressure: 0.1,
    };
    config
}

#[test]
fn test_non_overlapping_cells_do_not_move_via_displacement() {
    use alife::core::{cell_store::CellIndex, tick::TickExecutor};

    let config = two_cell_config_no_overlap();
    let mut exec = TickExecutor::new(config).unwrap();

    let pos_a_before = exec.world().cells().position(CellIndex::from_raw(0));
    let pos_b_before = exec.world().cells().position(CellIndex::from_raw(1));

    exec.step().unwrap();
    exec.step().unwrap();

    let pos_a_after = exec.world().cells().position(CellIndex::from_raw(0));
    let pos_b_after = exec.world().cells().position(CellIndex::from_raw(1));

    assert!(
        (pos_a_after.x() - pos_a_before.x()).abs() < 0.001,
        "Cell A must not move when no overlap: before={} after={}",
        pos_a_before.x(),
        pos_a_after.x()
    );
    assert!(
        (pos_b_after.x() - pos_b_before.x()).abs() < 0.001,
        "Cell B must not move when no overlap: before={} after={}",
        pos_b_before.x(),
        pos_b_after.x()
    );
}
// ─── Task 7: division_ready flag ─────────────────────────────────────────────

fn single_cell_config(radius: f32, target: f32) -> alife::core::config::RuntimeConfig {
    use alife::core::{
        config::{
            CellInitialConfig, EnvironmentConfig, GrowthConfig, LifecycleConfig, ResourceConfig,
            ResourceInteractionConfig, SpaceConfig, WorldConfig,
        },
        units::{
            CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
            ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
        },
    };
    let cell = CellInitialConfig {
        position: Position::new(32.0, 32.0),
        radius: Radius::new(radius).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::zero(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = alife::core::config::RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(1),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(0.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(0.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.0).unwrap(),
            energy_per_resource: 1.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.0).unwrap(),
            heat_warning_threshold: HeatAmount::new(80.0).unwrap(),
            heat_death_threshold: HeatAmount::new(100.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.0).unwrap(),
            waste_warning_threshold: WasteAmount::new(80.0).unwrap(),
            waste_death_threshold: WasteAmount::new(100.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap();
    config.growth = GrowthConfig {
        growth_cost_resource: ResourceAmount::new(999.0).unwrap(),
        growth_cost_energy: EnergyAmount::new(999.0).unwrap(),
        growth_target_radius: Radius::new(target).unwrap(),
        max_division_pressure: 0.5,
    };
    config.growth_enabled = false;
    config
}

#[test]
fn test_division_ready_false_below_target_radius() {
    use alife::core::{cell_store::CellIndex, tick::TickExecutor};
    // radius=1.0, growth_target_radius=2.0 -> after step: division_ready=false
    let mut exec = TickExecutor::new(single_cell_config(1.0, 2.0)).unwrap();
    exec.step().unwrap();
    let idx = CellIndex::from_raw(0);
    assert!(
        !exec.world().cells().runtime_flags(idx).division_ready,
        "division_ready must be false when radius < growth_target_radius"
    );
}

#[test]
fn test_division_ready_true_at_target_radius_low_pressure() {
    use alife::core::{cell_store::CellIndex, tick::TickExecutor};
    // radius=3.0, growth_target_radius=2.0, no neighbors -> pressure=0 -> ready=true
    let mut exec = TickExecutor::new(single_cell_config(3.0, 2.0)).unwrap();
    exec.step().unwrap();
    let idx = CellIndex::from_raw(0);
    assert!(
        exec.world().cells().runtime_flags(idx).division_ready,
        "division_ready must be true when radius >= target and pressure=0"
    );
}
