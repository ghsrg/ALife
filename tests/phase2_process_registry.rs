use alife::core::process::{MaterialCapability, ProcessId, ProcessSpec, ProcessStatus};

#[test]
fn test_every_process_id_has_registry_entry() {
    let all_ids = [
        ProcessId::MandatoryUpkeep,
        ProcessId::LocalResourceUptake,
        ProcessId::MetabolismEnergyConversion,
        ProcessId::MaterialSynthesis,
        ProcessId::GrowthResourceAllocation,
        ProcessId::Division,
        ProcessId::ContractileDisplacement,
    ];
    for id in all_ids {
        let spec = ProcessSpec::for_id(id);
        assert_eq!(spec.process_id, id, "Missing registry entry for {:?}", id);
    }
}

#[test]
fn test_division_is_now_status_in_phase_2d() {
    assert_eq!(
        ProcessSpec::for_id(ProcessId::Division).status,
        ProcessStatus::Now,
        "Division must be executable in Phase 2D"
    );
}

#[test]
fn test_uptake_requires_resource_uptake_capability() {
    assert!(
        ProcessSpec::for_id(ProcessId::LocalResourceUptake)
            .required_capabilities
            .contains(&MaterialCapability::ResourceUptake),
        "LocalResourceUptake must declare ResourceUptake capability requirement"
    );
}

// ─── Task 3: FeasibilityResult::Allowed payload tests ────────────────────────

fn minimal_config_with_resource(density: f32) -> alife::core::config::RuntimeConfig {
    use alife::core::{
        config::{
            CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
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
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
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
        ResourceConfig::new(vec![ResourceAmount::new(density).unwrap()], 0.0).unwrap(),
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

use alife::core::config::RuntimeConfig;

#[test]
fn test_feasibility_allowed_carries_accepted_amount() {
    use alife::core::{
        cell_store::CellIndex,
        process::{ActionCandidate, FeasibilityResult, ProcessId},
        tick::TickExecutor,
    };

    let executor = TickExecutor::new(minimal_config_with_resource(5.0)).unwrap();
    let idx = CellIndex::from_raw(0);
    let candidate = ActionCandidate {
        process_id: ProcessId::LocalResourceUptake,
        requested_amount: 0.3,
    };
    match executor.world().validate_feasibility(idx, &candidate) {
        FeasibilityResult::Allowed {
            accepted_amount, ..
        } => {
            assert!(
                accepted_amount > 0.0,
                "accepted_amount must be positive, got {}",
                accepted_amount
            );
            assert!(
                accepted_amount <= 0.3 + f32::EPSILON,
                "accepted_amount must not exceed requested 0.3, got {}",
                accepted_amount
            );
        }
        r => panic!("Expected Allowed, got {:?}", r),
    }
}

#[test]
fn test_feasibility_clamps_to_available_resource() {
    use alife::core::{
        cell_store::CellIndex,
        process::{ActionCandidate, FeasibilityResult, ProcessId},
        tick::TickExecutor,
    };

    let executor = TickExecutor::new(minimal_config_with_resource(0.1)).unwrap();
    let idx = CellIndex::from_raw(0);
    let candidate = ActionCandidate {
        process_id: ProcessId::LocalResourceUptake,
        requested_amount: 1.0,
    };
    match executor.world().validate_feasibility(idx, &candidate) {
        FeasibilityResult::Allowed {
            accepted_amount, ..
        } => {
            assert!(
                accepted_amount <= 0.1 + f32::EPSILON,
                "accepted_amount must be clamped to available 0.1, got {}",
                accepted_amount
            );
        }
        r => panic!("Expected Allowed, got {:?}", r),
    }
}

#[test]
fn test_diagnostics_records_metabolism_rejection_when_metabolic_material_zero() {
    use alife::core::{process::ProcessId, tick::TickExecutor};
    let mut config = minimal_config_with_resource(5.0);
    config.cell.initial_metabolic_material = alife::core::units::MaterialAmount::zero();
    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    let rejections = summary
        .diagnostics
        .rejections_by_process
        .get(&ProcessId::MetabolismEnergyConversion)
        .copied()
        .unwrap_or(0);
    assert!(
        rejections > 0,
        "Expected metabolism rejections, got {}",
        rejections
    );
}
