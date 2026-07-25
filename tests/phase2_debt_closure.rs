use alife::core::{
    cell_store::CellIndex,
    config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
    },
    process::{
        ActionCandidate, FeasibilityResult, ProcessId, ProcessSpec, ProcessStatus, RejectionReason,
    },
    tick::TickExecutor,
    units::{
        CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
        Seed, Tick, WasteAmount, WorldSize,
    },
};

fn minimal_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(32.0, 32.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(5.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(1.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    };
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(42),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(5.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.5).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.1,
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
            stress_energy_threshold: EnergyAmount::new(10.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.5,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn test_al_002_s17_ac01_joint_repair_disposition() {
    let executor = TickExecutor::new(minimal_config()).unwrap();
    let idx = CellIndex::from_raw(0);
    let candidate = ActionCandidate {
        process_id: ProcessId::JointRepair,
        requested_amount: 1.0,
    };

    // AL-002-S17-AC01: JointRepair is ProcessDisabled by canonical design for Phase 2
    let result = executor.world().validate_feasibility(idx, &candidate);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::ProcessDisabled)
    );

    let spec = ProcessSpec::for_id(ProcessId::JointRepair);
    assert_eq!(spec.status, ProcessStatus::Future);
}

#[test]
fn test_al_002_s17_ac02_repair_boundary_execution() {
    use alife::core::materials::MaterialSlot;

    let mut config = minimal_config();
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.max_amount_per_tick = 1.0;
    config.chemistry.materials = Vec::new();
    let mut executor = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    // Add generic resource and apply material damage so repair has target damage
    executor
        .world_mut()
        .cells_mut_for_commit()
        .add_resources_limited_by_capacity(idx, ResourceAmount::new(5.0).unwrap());
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(idx, MaterialSlot::Boundary, 0.5);

    let candidate = ActionCandidate {
        process_id: ProcessId::RepairBoundary,
        requested_amount: 0.5,
    };

    let result = executor.world().validate_feasibility(idx, &candidate);
    match result {
        FeasibilityResult::Allowed {
            accepted_amount, ..
        } => {
            assert!(accepted_amount > 0.0);
        }
        r => panic!(
            "Expected RepairBoundary to be Allowed when damage and resources exist, got {:?}",
            r
        ),
    }
}

#[test]
fn test_al_002_s17_ac03_boundary_retention_and_observer_heat_metrics() {
    let executor = TickExecutor::new(minimal_config()).unwrap();
    let world = executor.world();
    let idx = CellIndex::from_raw(0);

    // Boundary retention check: initial_boundary_material > 0
    let boundary = world.cells().boundary_material(idx).raw();
    assert!(boundary > 0.0);

    // Diagnostics check: Environment exposes current heat/temperature
    let heat = world.environment().heat().raw();
    assert!(heat >= 0.0);
}
