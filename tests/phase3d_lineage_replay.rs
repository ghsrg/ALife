use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, GenomeCopyingConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::lineage::{LineageEventKind, build_lineage_replay_summary};
use alife::core::process::{ActionCandidate, ProcessId};
use alife::core::stable_state_hash::StableStateHasher;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn lineage_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.5).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(30.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        capacity_limit: CapacityAmount::new(40.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(1),
            size: WorldSize::new(32.0, 32.0).unwrap(),
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
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.1).unwrap(),
            heat_warning_threshold: HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
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

fn division_candidate() -> ActionCandidate {
    ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 1.0,
    }
}

fn complete_genome_copy(executor: &mut TickExecutor) {
    let cell = CellIndex::from_raw(0);
    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();
    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();
}

#[test]
fn initial_cells_emit_founder_lineage_events() {
    let executor = TickExecutor::new(lineage_config()).unwrap();
    let events: Vec<_> = executor.world().lineage_events().iter_ordered().collect();

    assert_eq!(events.len(), executor.world().cells().len());
    assert_eq!(events[0].tick(), Tick::from_raw(0));
    assert_eq!(events[0].kind(), LineageEventKind::FounderCell);

    let founder = events[0].as_founder_cell().unwrap();
    assert_eq!(
        founder.cell_id,
        executor.world().cells().id_at(CellIndex::from_raw(0))
    );
    assert!(founder.genome_id.is_some());
    assert_eq!(
        founder.genome_template_id.as_ref().map(|id| id.as_str()),
        Some("balanced")
    );
}

#[test]
fn genome_copy_completion_records_parent_and_child_genome() {
    let mut executor = TickExecutor::new(lineage_config()).unwrap();
    let cell = CellIndex::from_raw(0);
    let parent_genome_id = executor.world().cells().genome_id(cell).unwrap();

    complete_genome_copy(&mut executor);

    let copied_genome_id = executor.world().cells().copied_genome_id(cell).unwrap();
    let copy = executor
        .world()
        .lineage_events()
        .iter_ordered()
        .find_map(|event| event.as_genome_copied())
        .unwrap();

    assert_eq!(copy.cell_id, executor.world().cells().id_at(cell));
    assert_eq!(copy.parent_genome_id, parent_genome_id);
    assert_eq!(copy.copied_genome_id, copied_genome_id);
    assert_eq!(copy.carrier_material_id, "genome_carrier_A");
    assert_eq!(copy.carrier_amount, 1.0);
    assert_eq!(copy.carrier_integrity, 1.0);
    assert!(copy.mutation_deltas.is_empty());
}

#[test]
fn forced_mutation_records_bounded_output_delta() {
    let mut config = lineage_config();
    config.genome_copying.mutation_rate = 1.0;
    let mut executor = TickExecutor::new(config).unwrap();

    complete_genome_copy(&mut executor);

    let copy = executor
        .world()
        .lineage_events()
        .iter_ordered()
        .find_map(|event| event.as_genome_copied())
        .unwrap();

    assert!(!copy.mutation_deltas.is_empty());
    for delta in &copy.mutation_deltas {
        assert_ne!(delta.before, delta.after);
        assert!(delta.before >= 0.0 && delta.before <= 1.0);
        assert!(delta.after >= 0.0 && delta.after <= 1.0);
    }
}

#[test]
fn division_lineage_event_reconstructs_parent_and_daughters() {
    let mut executor = TickExecutor::new(lineage_config()).unwrap();
    let cell = CellIndex::from_raw(0);
    let parent_id = executor.world().cells().id_at(cell);

    complete_genome_copy(&mut executor);
    let outcome = executor
        .world_mut()
        .execute_division(cell, &division_candidate())
        .unwrap();

    let division = executor
        .world()
        .lineage_events()
        .iter_ordered()
        .find_map(|event| event.as_cell_divided())
        .unwrap();

    assert_eq!(division.parent_cell_id, parent_id);
    assert_eq!(division.daughter_a_cell_id, outcome.daughter_a_id);
    assert_eq!(division.daughter_b_cell_id, outcome.daughter_b_id);
    assert_eq!(
        division.split_ratio,
        executor.world().config().division.split_ratio
    );
    assert_eq!(
        division.partition_loss_fraction,
        executor.world().config().division.partition_loss_fraction
    );
}

#[test]
fn division_lineage_event_records_genome_inheritance() {
    let mut executor = TickExecutor::new(lineage_config()).unwrap();
    let cell = CellIndex::from_raw(0);
    let parent_genome_id = executor.world().cells().genome_id(cell).unwrap();

    complete_genome_copy(&mut executor);
    let copied_genome_id = executor.world().cells().copied_genome_id(cell).unwrap();
    executor
        .world_mut()
        .execute_division(cell, &division_candidate())
        .unwrap();

    let division = executor
        .world()
        .lineage_events()
        .iter_ordered()
        .find_map(|event| event.as_cell_divided())
        .unwrap();

    assert_eq!(division.parent_genome_id, Some(parent_genome_id));
    assert_eq!(division.daughter_a_genome_id, Some(parent_genome_id));
    assert_eq!(division.daughter_b_genome_id, Some(copied_genome_id));
}

#[test]
fn death_lineage_event_records_cell_and_genome_at_death() {
    let mut config = lineage_config();
    config.cell.initial_energy = EnergyAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(1.0).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = CellIndex::from_raw(0);
    let cell_id = executor.world().cells().id_at(cell);
    let genome_id = executor.world().cells().genome_id(cell);

    executor.step().unwrap();

    let death = executor
        .world()
        .lineage_events()
        .iter_ordered()
        .find_map(|event| event.as_cell_died())
        .unwrap();
    assert_eq!(death.cell_id, cell_id);
    assert_eq!(death.genome_id, genome_id);
    assert_eq!(
        executor.world().cells().lifecycle_state(cell),
        LifecycleState::Dead
    );
}

#[test]
fn lineage_replay_marks_dead_cells_as_not_alive() {
    let mut config = lineage_config();
    config.cell.initial_energy = EnergyAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(1.0).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let cell_id = executor.world().cells().id_at(CellIndex::from_raw(0));

    executor.step().unwrap();

    let replay = build_lineage_replay_summary(executor.world().lineage_events());
    let record = replay.cell(cell_id).unwrap();
    assert!(!record.alive);
    assert_eq!(record.death_tick, Some(Tick::from_raw(0)));
}

#[test]
fn lineage_replay_is_deterministic_for_same_seed_and_config() {
    let mut first = TickExecutor::new(lineage_config()).unwrap();
    let mut second = TickExecutor::new(lineage_config()).unwrap();

    complete_genome_copy(&mut first);
    complete_genome_copy(&mut second);
    first
        .world_mut()
        .execute_division(CellIndex::from_raw(0), &division_candidate())
        .unwrap();
    second
        .world_mut()
        .execute_division(CellIndex::from_raw(0), &division_candidate())
        .unwrap();

    assert_eq!(
        first.world().lineage_events(),
        second.world().lineage_events()
    );
    assert_eq!(
        build_lineage_replay_summary(first.world().lineage_events()),
        build_lineage_replay_summary(second.world().lineage_events())
    );
}

#[test]
fn lineage_replay_does_not_change_stable_behavior_hash() {
    let mut executor = TickExecutor::new(lineage_config()).unwrap();
    complete_genome_copy(&mut executor);
    let before = StableStateHasher::hash_world(executor.world());

    let _replay = build_lineage_replay_summary(executor.world().lineage_events());

    assert_eq!(StableStateHasher::hash_world(executor.world()), before);
}

#[test]
fn genome_runtime_and_feasibility_do_not_depend_on_lineage_replay() {
    for path in [
        "src/core/genome.rs",
        "src/core/process.rs",
        "src/core/action_plan.rs",
    ] {
        let source = std::fs::read_to_string(path).unwrap();
        assert!(!source.contains("LineageReplay"));
        assert!(!source.contains("build_lineage_replay_summary"));
        assert!(!source.contains("lineage_events()"));
    }
}
