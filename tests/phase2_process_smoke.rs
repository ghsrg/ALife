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
