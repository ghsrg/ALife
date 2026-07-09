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
        // Verify metrics summary has recorded the attempts and rejections
        assert_eq!(summary.metrics.process_attempts, 4);
        assert_eq!(summary.metrics.process_rejections, 3);
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

        // Verify metrics summary has recorded the attempts and rejections
        assert_eq!(summary.metrics.process_attempts, 4);
        assert_eq!(summary.metrics.process_rejections, 2);
    }
}

#[test]
fn cell_collapses_if_metabolism_capability_is_missing() {
    use alife::core::cell_store::CellIndex;
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;

    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let cell_idx = CellIndex::from_raw(0);

    exec.world_mut()
        .cells_mut_for_commit()
        .set_metabolic_material(cell_idx, alife::core::units::MaterialAmount::zero());

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
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;

    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let cell_idx = CellIndex::from_raw(0);

    // Zero out transport material to remove ResourceUptake capability
    exec.world_mut()
        .cells_mut_for_commit()
        .set_transport_material(cell_idx, alife::core::units::MaterialAmount::zero());

    // Run until collapse (or max configured tick of 10)
    let summary = exec.run_until_configured_tick().unwrap();

    // Since resource uptake is missing, metabolism has no internal resources to convert to energy.
    // By tick 3, energy is depleted, and the cell collapses.
    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
}

#[test]
fn test_reachability_division_and_birth_events() {
    use alife::core::cell_store::CellIndex;
    use alife::core::config::CellInitialConfig;
    use alife::core::tick::TickExecutor;
    use alife::core::units::{EnergyAmount, MaterialAmount, Radius, ResourceAmount};

    let base = base_test_config();
    let cell = CellInitialConfig {
        position: alife::core::units::Position::new(8.0, 8.0),
        radius: Radius::new(2.0).unwrap(), // Target radius for division
        initial_energy: EnergyAmount::new(90.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: alife::core::units::CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(5.0).unwrap(),
        initial_transport_material: MaterialAmount::new(5.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(5.0).unwrap(),
        initial_storage_material: MaterialAmount::new(5.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(5.0).unwrap(),
        initial_structural_material: MaterialAmount::new(20.0).unwrap(),
        initial_repair_material: MaterialAmount::new(5.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(5.0).unwrap(),
    };

    let mut config = base.with_cells(vec![cell]);
    config.growth_enabled = true;
    config.growth.growth_target_radius = Radius::new(2.0).unwrap();
    config.division.enabled = true;
    config.division.split_ratio = 0.5;
    config.division.partition_loss_fraction = 0.1;
    config.division.daughter_spacing = 1.0;
    config.division.min_daughter_radius = Radius::new(0.5).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();

    // Manually mark division_ready = true
    let idx = CellIndex::from_raw(0);
    let mut flags = exec.world().cells().runtime_flags(idx);
    flags.division_ready = true;
    exec.world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(idx, flags);

    let summary = exec.step().unwrap();

    // Verify division occurred
    assert_eq!(summary.metrics.divisions_count, 1);
    assert_eq!(summary.metrics.births_count, 1);

    // Verify daughter cells partition
    assert_eq!(exec.world().cells().len(), 2);
    let d1 = CellIndex::from_raw(0);
    let d2 = CellIndex::from_raw(1);
    assert!(exec.world().cells().radius(d1).raw() < 2.0);
    assert!(exec.world().cells().radius(d2).raw() < 2.0);

    // Verify events were logged
    let mut has_divided = false;
    let mut has_born = false;
    for ev in exec.world().events().iter_ordered() {
        if ev.tick == alife::core::units::Tick::from_raw(0) {
            if ev.kind == alife::core::events::EventKind::CellDivided {
                has_divided = true;
            }
            if ev.kind == alife::core::events::EventKind::CellBorn {
                has_born = true;
            }
        }
    }
    assert!(has_divided, "CellDivided event should be present");
    assert!(has_born, "CellBorn event should be present");
}

#[test]
fn test_reachability_death_and_decomposition() {
    use alife::core::cell_store::CellIndex;
    use alife::core::config::CellInitialConfig;
    use alife::core::tick::TickExecutor;
    use alife::core::units::{EnergyAmount, MaterialAmount, Radius, ResourceAmount};

    let base = base_test_config();
    let cell = CellInitialConfig {
        position: alife::core::units::Position::new(8.0, 8.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(1.0).unwrap(),
        energy_capacity: EnergyAmount::new(10.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(5.0).unwrap(), // high cost to trigger death
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: alife::core::units::CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(10.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::zero(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };

    let mut config = base.with_cells(vec![cell]);
    config.decomposition.enabled = true;
    config.decomposition.resource_layer_index = 0;
    config.decomposition.resources_per_tick = ResourceAmount::new(5.0).unwrap();
    config.decomposition.materials_per_tick = MaterialAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();

    // Step 1: Cell dies from unpaid mandatory cost
    let summary1 = exec.step().unwrap();
    let idx = CellIndex::from_raw(0);
    assert_eq!(
        exec.world().cells().lifecycle_state(idx),
        alife::core::cell_store::LifecycleState::Dead
    );
    assert_eq!(summary1.metrics.decomposed_cells_count, 0);

    // Verify death event was emitted
    let mut has_dead_event = false;
    for ev in exec.world().events().iter_ordered() {
        if ev.tick == alife::core::units::Tick::from_raw(0)
            && ev.kind == alife::core::events::EventKind::CellDead
        {
            has_dead_event = true;
        }
    }
    assert!(has_dead_event, "CellDead event should be present");

    // Step 2: Second step of decomposition (complete)
    let summary2 = exec.step().unwrap();
    assert_eq!(summary2.metrics.decomposed_cells_count, 1);

    // Verify decomposed event was emitted
    let mut has_decomposed_event = false;
    for ev in exec.world().events().iter_ordered() {
        if ev.tick == alife::core::units::Tick::from_raw(1)
            && ev.kind == alife::core::events::EventKind::CellDecomposed
        {
            has_decomposed_event = true;
        }
    }
    assert!(
        has_decomposed_event,
        "CellDecomposed event should be present"
    );
}
