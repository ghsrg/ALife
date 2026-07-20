use alife::core::cell_store::{CellIndex, EnergyBuffer};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, GenomeCopyingConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
use alife::core::stable_state_hash::StableStateHasher;
use alife::core::tick::TickExecutor;
use alife::core::units::{CapacityAmount, EnergyAmount, Position, Radius, ResourceAmount};
use alife::runner::config_parser::RawScenarioConfig;

fn base_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.5).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(30.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        capacity_limit: CapacityAmount::new(40.0).unwrap(),
        initial_boundary_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: alife::core::units::MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: alife::core::units::MaterialAmount::zero(),
        initial_sensory_material: alife::core::units::MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: alife::core::units::Tick::from_raw(1),
            seed: alife::core::units::Seed::from_raw(1),
            size: alife::core::units::WorldSize::new(32.0, 32.0).unwrap(),
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
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 1.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
        EnvironmentConfig {
            heat_current: alife::core::units::HeatAmount::zero(),
            heat_generated_per_tick: alife::core::units::HeatAmount::zero(),
            heat_dissipation_rate: alife::core::units::HeatAmount::new(0.1).unwrap(),
            heat_warning_threshold: alife::core::units::HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: alife::core::units::HeatAmount::new(40.0).unwrap(),
            waste_current: alife::core::units::WasteAmount::zero(),
            waste_generated_per_tick: alife::core::units::WasteAmount::zero(),
            waste_sink_rate: alife::core::units::WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: alife::core::units::WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: alife::core::units::WasteAmount::new(40.0).unwrap(),
        },
        alife::core::config::LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 0.5,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap();
    config.growth = alife::core::config::GrowthConfig {
        growth_cost_resource: ResourceAmount::zero(),
        growth_cost_energy: EnergyAmount::zero(),
        growth_target_radius: Radius::new(1.5).unwrap(),
        max_division_pressure: 0.5,
    };
    config.growth_enabled = true;
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::zero();
    config.genome_copying = GenomeCopyingConfig {
        enabled: true,
        energy_cost_per_step: EnergyAmount::new(0.5).unwrap(),
        carrier_resource_cost_per_step: ResourceAmount::new(0.5).unwrap(),
        progress_per_step: 0.5,
        mutation_rate: 0.0,
        mutation_step: 0.05,
    };
    config.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.0,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            vec![
                (
                    GenomeOutputId::GenomeCopyingPriority,
                    GenomeOutputValue::new(1.0),
                ),
                (
                    GenomeOutputId::ResourceUptakePriority,
                    GenomeOutputValue::new(0.2),
                ),
            ],
        )
        .unwrap(),
    );
    config.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];
    config
}

fn copying_candidate() -> ActionCandidate {
    ActionCandidate {
        process_id: ProcessId::GenomeCopying,
        requested_amount: 1.0,
    }
}

#[test]
fn genome_copying_feasibility_requires_physical_inputs() {
    let mut executor = TickExecutor::new(base_config()).unwrap();
    let cell = CellIndex::from_raw(0);

    assert!(matches!(
        executor
            .world()
            .validate_feasibility(cell, &copying_candidate()),
        FeasibilityResult::Allowed { .. }
    ));

    executor.world_mut().cells_mut_for_commit().set_energy(
        cell,
        EnergyBuffer::new(EnergyAmount::zero(), EnergyAmount::new(30.0).unwrap()),
    );
    assert_eq!(
        executor
            .world()
            .validate_feasibility(cell, &copying_candidate()),
        FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
    );
}

#[test]
fn genome_copying_progress_consumes_energy_and_carrier_resource() {
    let mut executor = TickExecutor::new(base_config()).unwrap();
    let cell = CellIndex::from_raw(0);
    let before_energy = executor.world().cells().energy(cell).current().raw();
    let before_resources = executor.world().cells().generic_resource_amount(cell).raw();

    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();

    let cells = executor.world().cells();
    assert_eq!(cells.genome_copy_progress(cell), 0.5);
    assert_eq!(cells.copied_genome_carrier_amount(cell), 0.5);
    assert_eq!(cells.energy(cell).current().raw(), before_energy - 0.5);
    assert_eq!(
        cells.generic_resource_amount(cell).raw(),
        before_resources - 0.5
    );
    assert!(cells.copied_genome_id(cell).is_none());
}

#[test]
fn genome_copying_completion_creates_world_owned_copy_without_mutation_by_default() {
    let mut executor = TickExecutor::new(base_config()).unwrap();
    let cell = CellIndex::from_raw(0);
    let parent_id = executor.world().cells().genome_id(cell).unwrap();

    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();
    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();

    let copied_id = executor.world().cells().copied_genome_id(cell).unwrap();
    assert_ne!(copied_id, parent_id);
    assert_eq!(
        executor.world().genome(copied_id).unwrap().outputs,
        executor.world().genome(parent_id).unwrap().outputs
    );
}

#[test]
fn forced_genome_copy_mutation_is_deterministic_and_bounded() {
    let mut first = base_config();
    first.genome_copying.progress_per_step = 1.0;
    first.genome_copying.mutation_rate = 1.0;
    first.genome_copying.mutation_step = 0.25;
    let second = first.clone();

    let mut first_exec = TickExecutor::new(first).unwrap();
    let mut second_exec = TickExecutor::new(second).unwrap();
    let cell = CellIndex::from_raw(0);

    first_exec
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();
    second_exec
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();

    let first_copy = first_exec
        .world()
        .genome(first_exec.world().cells().copied_genome_id(cell).unwrap())
        .unwrap();
    let second_copy = second_exec
        .world()
        .genome(second_exec.world().cells().copied_genome_id(cell).unwrap())
        .unwrap();
    assert_eq!(first_copy.outputs, second_copy.outputs);
    assert!(
        first_copy
            .outputs
            .iter()
            .all(|(_, value)| (-1.0..=1.0).contains(&value.raw()))
    );
    assert_ne!(
        first_copy
            .output(GenomeOutputId::ResourceUptakePriority)
            .unwrap()
            .raw(),
        0.2
    );
}

#[test]
fn division_of_genome_cell_requires_completed_genome_copy() {
    let mut config = base_config();
    config.genome_copying.progress_per_step = 1.0;
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = CellIndex::from_raw(0);
    let division = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 1.0,
    };

    assert_eq!(
        executor.world().validate_feasibility(cell, &division),
        FeasibilityResult::Rejected(RejectionReason::MissingGenomeCopy)
    );

    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();
    assert!(matches!(
        executor.world().validate_feasibility(cell, &division),
        FeasibilityResult::Allowed { .. }
    ));

    let outcome = executor
        .world_mut()
        .execute_division(cell, &division)
        .unwrap();
    assert!(
        executor
            .world()
            .cells()
            .genome_id(outcome.daughter_b_index)
            .is_some()
    );
    assert_eq!(
        executor
            .world()
            .cells()
            .genome_copy_progress(outcome.daughter_a_index),
        0.0
    );
}

#[test]
fn conservative_genome_copying_scenario_parses_for_sweeper() {
    let raw = include_str!("../config/scenarios/genome/phase3c_genome_copying_conservative.toml");
    let config = RawScenarioConfig::parse(raw).unwrap();

    assert!(config.genome_copying.enabled);
    assert!(config.genome_copying.progress_per_step > 0.0);
    assert!(config.genome_copying.progress_per_step <= 0.25);
    assert!(config.genome_copying.mutation_rate <= 0.01);

    let mut first = TickExecutor::new(config.clone()).unwrap();
    let mut second = TickExecutor::new(config).unwrap();
    for _ in 0..8 {
        first.step().unwrap();
        second.step().unwrap();
    }
    assert_eq!(
        StableStateHasher::hash_world(first.world()),
        StableStateHasher::hash_world(second.world())
    );
}
