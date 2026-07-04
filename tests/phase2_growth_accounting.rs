// Task 5 - RED test: growth must increase ONLY structural_material

fn make_growth_config() -> alife::core::config::RuntimeConfig {
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
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(100.0).unwrap(),
        energy_capacity: EnergyAmount::new(200.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(50.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        // No contractile material - Contractility must not appear after growth
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    };
    let mut config = alife::core::config::RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.5).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.1).unwrap(),
            heat_warning_threshold: HeatAmount::new(80.0).unwrap(),
            heat_death_threshold: HeatAmount::new(100.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: WasteAmount::new(80.0).unwrap(),
            waste_death_threshold: WasteAmount::new(100.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(5.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap();
    config.growth = GrowthConfig {
        growth_cost_resource: ResourceAmount::new(2.0).unwrap(),
        growth_cost_energy: EnergyAmount::new(1.0).unwrap(),
        growth_target_radius: Radius::new(5.0).unwrap(),
        max_division_pressure: 0.5,
    };
    config.growth_enabled = true;
    config
}

#[test]
fn test_growth_increases_only_structural_material() {
    use alife::core::{
        cell_store::CellIndex,
        process::{ActionCandidate, ProcessId},
        tick::TickExecutor,
    };

    let mut exec = TickExecutor::new(make_growth_config()).unwrap();
    let idx = CellIndex::from_raw(0);

    let before_transport = exec.world().cells().transport_material(idx).raw();
    let before_metabolic = exec.world().cells().metabolic_material(idx).raw();
    let before_contractile = exec.world().cells().contractile_material(idx).raw();
    let before_structural = exec.world().cells().structural_material(idx).raw();

    let action = ActionCandidate {
        process_id: ProcessId::GrowthResourceAllocation,
        requested_amount: 1.0,
    };
    exec.world_mut().execute_growth(idx, &action).unwrap();

    assert!(
        exec.world().cells().structural_material(idx).raw() > before_structural,
        "structural_material must increase after growth"
    );
    assert_eq!(
        exec.world().cells().transport_material(idx).raw(),
        before_transport,
        "transport_material must not change during growth"
    );
    assert_eq!(
        exec.world().cells().metabolic_material(idx).raw(),
        before_metabolic,
        "metabolic_material must not change during growth"
    );
    assert_eq!(
        exec.world().cells().contractile_material(idx).raw(),
        before_contractile,
        "contractile_material must not change during growth"
    );
}

#[test]
fn test_growth_increases_radius() {
    use alife::core::{
        cell_store::CellIndex,
        process::{ActionCandidate, ProcessId},
        tick::TickExecutor,
    };

    let mut exec = TickExecutor::new(make_growth_config()).unwrap();
    let idx = CellIndex::from_raw(0);
    let before_radius = exec.world().cells().radius(idx).raw();

    let action = ActionCandidate {
        process_id: ProcessId::GrowthResourceAllocation,
        requested_amount: 1.0,
    };
    exec.world_mut().execute_growth(idx, &action).unwrap();

    let after_radius = exec.world().cells().radius(idx).raw();
    assert!(
        after_radius > before_radius,
        "radius must increase after growth: before={} after={}",
        before_radius,
        after_radius
    );
}

#[test]
fn test_growth_does_not_grant_contractility() {
    use alife::core::{
        cell_store::CellIndex,
        process::{ActionCandidate, MaterialCapability, ProcessId},
        tick::TickExecutor,
    };

    let mut exec = TickExecutor::new(make_growth_config()).unwrap();
    let idx = CellIndex::from_raw(0);

    assert_eq!(exec.world().cells().contractile_material(idx).raw(), 0.0);
    assert!(
        !exec
            .world()
            .cells()
            .has_capability(idx, MaterialCapability::Contractility),
        "Contractility must be absent when contractile_material=0"
    );

    let action = ActionCandidate {
        process_id: ProcessId::GrowthResourceAllocation,
        requested_amount: 1.0,
    };
    for _ in 0..5 {
        exec.world_mut().execute_growth(idx, &action).unwrap();
    }

    assert!(
        !exec
            .world()
            .cells()
            .has_capability(idx, MaterialCapability::Contractility),
        "Contractility must NOT be granted after 5x growth when contractile_material was zero"
    );
    assert_eq!(
        exec.world().cells().contractile_material(idx).raw(),
        0.0,
        "contractile_material must remain zero after 5x growth"
    );
}
