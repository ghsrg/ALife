use alife::core::config::RuntimeConfig;
use alife::core::summary::SurvivalResult;
use alife::core::tick::TickExecutor;
use alife::core::units::ResourceAmount;

fn base_test_config() -> RuntimeConfig {
    use alife::core::config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
    };
    use alife::core::units::{
        CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
        Seed, Tick, WasteAmount, WorldSize,
    };

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
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
fn tick_executor_records_process_attempts_and_rejections() {
    // Case 1: Uptake is 0.0, so metabolism is rejected due to InsufficientResources
    {
        let mut config = base_test_config();
        config.resource_interaction.enabled = true;
        config.resource_interaction.max_uptake_per_tick = ResourceAmount::zero();
        config.resource_interaction.metabolism_resource_per_tick =
            ResourceAmount::new(1.0).unwrap();
        config.resource_interaction.energy_per_resource = 3.0;

        let mut exec = TickExecutor::new(config).unwrap();
        let summary = exec.step().unwrap();
        assert_eq!(summary.survival_result, SurvivalResult::Stable);

        // Verify metrics summary has recorded the attempt and rejection
        assert_eq!(summary.metrics.process_attempts, 2);
        assert_eq!(summary.metrics.process_rejections, 1);
    }

    // Case 2: Uptake is positive (2.0), so metabolism succeeds (0 rejections)
    {
        let mut config = base_test_config();
        config.resource_interaction.enabled = true;
        config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
        config.resource_interaction.metabolism_resource_per_tick =
            ResourceAmount::new(1.0).unwrap();
        config.resource_interaction.energy_per_resource = 3.0;

        let mut exec = TickExecutor::new(config).unwrap();
        let summary = exec.step().unwrap();
        assert_eq!(summary.survival_result, SurvivalResult::Stable);

        // Verify metrics summary has recorded the attempt and 0 rejections
        assert_eq!(summary.metrics.process_attempts, 2);
        assert_eq!(summary.metrics.process_rejections, 0);
    }
}

#[test]
fn cell_collapses_if_metabolism_capability_is_missing() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::MaterialCapability;
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;

    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let cell_idx = CellIndex::from_raw(0);

    // Strip metabolism capability
    exec.world_mut()
        .cells_mut_for_commit()
        .strip_capability_for_test(cell_idx, MaterialCapability::Metabolism);

    // Run until collapse (or max configured tick of 10)
    let summary = exec.run_until_configured_tick().unwrap();

    // Since metabolism is missing, energy cannot be generated.
    // The cell has 5.0 initial energy, and pays 2.0 energy mandatory cost per tick.
    // By tick 3, energy is depleted, and the cell collapses.
    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
}

#[test]
fn cell_collapses_if_resource_uptake_capability_is_missing() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::MaterialCapability;
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;

    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let cell_idx = CellIndex::from_raw(0);

    // Strip resource uptake capability
    exec.world_mut()
        .cells_mut_for_commit()
        .strip_capability_for_test(cell_idx, MaterialCapability::ResourceUptake);

    // Run until collapse (or max configured tick of 10)
    let summary = exec.run_until_configured_tick().unwrap();

    // Since resource uptake is missing, metabolism has no internal resources to convert to energy.
    // By tick 3, energy is depleted, and the cell collapses.
    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
}
