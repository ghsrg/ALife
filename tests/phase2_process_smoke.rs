use alife::core::process::{MaterialCapability, MaterialCapabilityFlags};

#[test]
fn material_capabilities_flags_work() {
    let flags = MaterialCapabilityFlags {
        boundary_permeability: true,
        resource_uptake: true,
        metabolism: false,
        storage_capacity: true,
        material_synthesis: false,
        structural_growth: false,
        repair: false,
        contractility: false,
        resource_sensing: false,
        pressure_sensing: false,
        damage_sensing: false,
    };
    assert!(flags.has(MaterialCapability::BoundaryPermeability));
    assert!(!flags.has(MaterialCapability::Metabolism));
}

#[test]
fn cell_inventory_queries_capabilities_based_on_material_amounts() {
    use alife::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
    use alife::core::units::{
        CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
    };

    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        transport_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        metabolic_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        storage_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        synthesis_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        structural_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        repair_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        contractile_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        sensory_material: MaterialAmount::new(5.0 / 9.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    let idx = alife::core::cell_store::CellIndex::from_raw(0);
    assert!(cells.has_capability(idx, MaterialCapability::Metabolism));
}

fn base_test_config() -> alife::core::config::RuntimeConfig {
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
fn feasibility_validates_uptake_and_metabolism() {
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};

    let base = base_test_config();
    let exec = alife::core::tick::TickExecutor::new(base).unwrap();
    let idx = alife::core::cell_store::CellIndex::from_raw(0);

    let candidate_uptake = ActionCandidate {
        process_id: ProcessId::LocalResourceUptake,
        requested_amount: 1.0,
    };
    let result = exec.world().validate_feasibility(idx, &candidate_uptake);
    assert!(matches!(result, FeasibilityResult::Feasible));

    let candidate_metabolism = ActionCandidate {
        process_id: ProcessId::MetabolismEnergyConversion,
        requested_amount: 1.0,
    };
    let result = exec
        .world()
        .validate_feasibility(idx, &candidate_metabolism);
    assert!(matches!(
        result,
        FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
    ));
}

#[test]
fn tick_executor_drives_uptake_and_metabolism_via_processes() {
    use alife::core::cell_store::CellIndex;
    use alife::core::summary::SurvivalResult;
    use alife::core::tick::TickExecutor;
    use alife::core::units::ResourceAmount;

    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();

    // Step 1: Uptake local resource
    let summary = exec.step().unwrap();
    assert_eq!(summary.survival_result, SurvivalResult::Stable);

    let cell_res = exec.world().cells().resource_amount(CellIndex::from_raw(0));
    assert!(cell_res.raw() > 0.0); // Uptake executed.
}

#[test]
fn test_synthesis_process_feasibility_and_execution() {
    use alife::core::cell_store::{CellIndex, EnergyBuffer};
    use alife::core::process::{
        ActionCandidate, FeasibilityResult, MaterialCapability, ProcessId, RejectionReason,
    };
    use alife::core::units::{EnergyAmount, MaterialAmount, ResourceAmount};

    let mut config = base_test_config();
    // Configure synthesis costs
    config.synthesis.cost_resource = ResourceAmount::new(1.0).unwrap();
    config.synthesis.cost_energy = EnergyAmount::new(5.0).unwrap();
    config.cell.initial_synthesis_material = MaterialAmount::new(1.0).unwrap();

    let mut exec = alife::core::tick::TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let candidate = ActionCandidate {
        process_id: ProcessId::MaterialSynthesis,
        requested_amount: 1.0,
    };

    // 1. Initially should be rejected due to insufficient resources (initial cell has 0.0 resources)
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
    );

    // 2. Add enough resources but reduce energy below cost_energy (5.0)
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_resources(idx, ResourceAmount::new(2.0).unwrap());
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(4.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
    }

    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
    );

    // 3. Make cell lack MaterialSynthesis capability
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(5.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
        cells.set_synthesis_material(idx, MaterialAmount::zero());
    }
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::MissingCapability(
            MaterialCapability::MaterialSynthesis
        ))
    );

    // 4. Restore capability, energy, resources, and execute synthesis
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_synthesis_material(idx, MaterialAmount::new(1.0).unwrap());
        cells.set_resources(idx, ResourceAmount::new(2.0).unwrap());
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(6.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
    }
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(result, FeasibilityResult::Feasible);

    let old_structural = exec.world().cells().structural_material(idx).raw();

    // Execute synthesis
    exec.world_mut().execute_synthesis(idx).unwrap();

    // Check post-conditions:
    // Resources: 2.0 - 1.0 = 1.0
    // Energy: 6.0 - 5.0 = 1.0
    // Structural material: old_structural + 1.0
    assert_eq!(exec.world().cells().resource_amount(idx).raw(), 1.0);
    assert_eq!(exec.world().cells().energy(idx).current().raw(), 1.0);
    assert_eq!(
        exec.world().cells().structural_material(idx).raw(),
        old_structural + 1.0
    );
}

#[test]
fn test_displacement_feasibility_and_execution() {
    use alife::core::cell_store::{CellIndex, EnergyBuffer};
    use alife::core::process::{
        ActionCandidate, FeasibilityResult, MaterialCapability, ProcessId, RejectionReason,
    };
    use alife::core::units::{EnergyAmount, MaterialAmount, Position};

    let mut config = base_test_config();
    config.contractility.energy_cost = EnergyAmount::new(1.0).unwrap();
    config.contractility.force_factor = 0.5;
    config.cell.initial_contractile_material = MaterialAmount::new(1.0).unwrap();

    // Use two cells overlapping to create push conditions
    let mut cell1 = config.cell;
    cell1.position = Position::new(5.0, 5.0);

    let mut cell2 = cell1;
    cell2.position = Position::new(6.5, 5.0);

    let cells_list = vec![cell1, cell2];
    config = config.with_cells(cells_list);

    let mut exec = alife::core::tick::TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let candidate = ActionCandidate {
        process_id: ProcessId::ContractileDisplacement,
        requested_amount: 1.0,
    };

    // 1. Initially, pressure is 0.0, so should be rejected with NoPressure
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::NoPressure)
    );

    // 2. Set pressure > 0.0 but energy below energy_cost (1.0)
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_contact_pressure(idx, 0.5);
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(0.5).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
    }
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
    );

    // 3. Make cell lack Contractility capability
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(5.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
        cells.set_contractile_material(idx, MaterialAmount::zero());
    }
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::MissingCapability(
            MaterialCapability::Contractility
        ))
    );

    // 4. Restore capability and execute displacement
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_contractile_material(idx, MaterialAmount::new(1.0).unwrap());
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(5.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
    }
    let result = exec.world().validate_feasibility(idx, &candidate);
    assert_eq!(result, FeasibilityResult::Feasible);

    // Execute displacement
    exec.world_mut().execute_displacement(idx).unwrap();

    // Verify displacement shift:
    // dx = -1.5, dist = 1.5, overlap = 0.5, push_x = -0.5.
    // shift = push_x * contractile_material * force_factor = -0.5 * 1.0 * 0.5 = -0.25.
    // new_x = 5.0 - 0.25 = 4.75.
    assert_eq!(exec.world().cells().position(idx).x(), 4.75);
    assert_eq!(exec.world().cells().position(idx).y(), 5.0);
    // Energy deducted: 5.0 - 1.0 = 4.0
    assert_eq!(exec.world().cells().energy(idx).current().raw(), 4.0);

    // 5. Test clamping: set position close to boundary and push left by updating other cell's position
    {
        let cells = exec.world_mut().cells_mut_for_commit();
        cells.set_position(idx, Position::new(1.1, 5.0));
        let other_idx = CellIndex::from_raw(1);
        cells.set_position(other_idx, Position::new(2.6, 5.0));
        cells.set_energy(
            idx,
            EnergyBuffer::new(
                EnergyAmount::new(5.0).unwrap(),
                EnergyAmount::new(10.0).unwrap(),
            ),
        );
    }
    // Execute displacement again
    exec.world_mut().execute_displacement(idx).unwrap();
    // Clamped to cell_rad = 1.0
    assert_eq!(exec.world().cells().position(idx).x(), 1.0);
}
