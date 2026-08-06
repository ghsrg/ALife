use alife::core::cell_store::LifecycleState;
use alife::core::ids::ResourceTypeId;
use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
use alife::core::tick::TickExecutor;
use alife::core::units::{Position, ResourceAmount};
use alife::runner::config_parser::{ParseError, RawScenarioConfig};

fn fixture() -> String {
    r#"
scenario_id = "phase3g"
seed = 8
tick_count = 10
[world]
size = [32.0, 32.0]
[space]
spatial_grid_size = 8.0
[resources]
resource_type_ids = ["nucleotide_precursor", "phosphate", "short_peptide", "catalyst_mineral", "inert_waste"]
initial_distribution = [10.0, 10.0, 10.0, 10.0, 0.0]
optional_decay_rate = 0.0
[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nucleotide_precursor = 3.0, phosphate = 2.0, short_peptide = 1.0, catalyst_mineral = 1.0, inert_waste = 0.0 }
initial_materials = { synthesis = 1.0, repair = 1.0 }
initial_energy = 8.0
energy_capacity = 10.0
mandatory_cost_per_tick = 0.0
capacity_limit = 20.0
[cell.genome]
template = "balanced"
[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0
[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 2.0
[genome_copying]
enabled = true
energy_cost_per_step = 0.5
carrier_resource_cost_per_step = 0.0
progress_per_step = 0.5
mutation_rate = 0.0
mutation_step = 0.05

[genome_physical_accounting.copying]
carrier_material_id = "genome_carrier_matrix"
carrier_output_amount_per_step = 0.5
precursor_requirements = { nucleotide_precursor = 1.0, phosphate = 0.5 }
waste_outputs = { inert_waste = 0.2 }

[genome_physical_accounting.recombination]
energy_cost = 4.0
precursor_requirements = { nucleotide_precursor = 0.6, phosphate = 0.4, short_peptide = 0.4, catalyst_mineral = 0.2 }
waste_outputs = { inert_waste = 0.3 }

[decomposition]
enabled = true
resource_layer_index = 4
resources_per_tick = 0.0
materials_per_tick = 2.0
remove_when_empty = false

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = 1
regulatory_depth = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_matrix"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
genome_copying_priority = 1.0
genome_recombination_priority = 0.5

[chemistry.resources.nucleotide_precursor]
volume = 1.0
diffusion_rate = 0.25
energy_value = 0.2
decay_rate = 0.01
reactivity_profile = "reactive"
permeability = "passive"
tags = ["structural_precursor", "dissolved"]
material_profile = { volume = 0.45, stability = 0.55, strength = 0.3, energy_capacity = 0.3, permeability = 0.55, durability = 0.5 }
material_capabilities = { genome_copying = 0.8, material_synthesis = 0.3 }

[chemistry.resources.phosphate]
volume = 1.0
diffusion_rate = 0.28
energy_value = 0.1
decay_rate = 0.0
reactivity_profile = "stable"
permeability = "passive"
tags = ["structural_precursor", "dissolved"]
material_profile = { volume = 0.4, stability = 0.6, strength = 0.35, energy_capacity = 0.25, permeability = 0.55, durability = 0.55 }
material_capabilities = { genome_copying = 0.4, material_synthesis = 0.2 }

[chemistry.resources.short_peptide]
volume = 1.0
diffusion_rate = 0.25
energy_value = 0.2
decay_rate = 0.01
reactivity_profile = "reactive"
permeability = "passive"
tags = ["structural_precursor", "dissolved"]
material_profile = { volume = 0.5, stability = 0.55, strength = 0.45, energy_capacity = 0.2, permeability = 0.55, durability = 0.5 }
material_capabilities = { genome_copying = 0.2 }

[chemistry.resources.catalyst_mineral]
volume = 1.0
diffusion_rate = 0.12
energy_value = 0.0
decay_rate = 0.0
reactivity_profile = "stable"
permeability = "blocked"
tags = ["structural_precursor"]
material_profile = { volume = 0.55, stability = 0.8, strength = 0.55, energy_capacity = 0.0, permeability = 0.25, durability = 0.8 }
material_capabilities = { genome_copying = 0.3 }

[chemistry.resources.inert_waste]
volume = 1.0
diffusion_rate = 0.1
energy_value = 0.0
decay_rate = 0.01
reactivity_profile = "stable"
permeability = "blocked"
tags = ["waste"]
material_profile = { volume = 0.25, stability = 0.6, strength = 0.2, energy_capacity = 0.0, permeability = 0.15, durability = 0.6 }
material_capabilities = {}
"#
    .to_string()
}

#[test]
fn parses_physical_genome_precursor_requirements() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();

    let accounting = config.genome_physical_accounting;
    let copying = accounting.copying.expect("copying accounting");
    assert_eq!(copying.carrier_material_id, "genome_carrier_matrix");
    assert_eq!(copying.carrier_output_amount_per_step.raw(), 0.5);
    assert_eq!(
        copying.precursor_requirements,
        vec![
            (
                ResourceTypeId::from_raw(0),
                alife::core::units::ResourceAmount::new(1.0).unwrap()
            ),
            (
                ResourceTypeId::from_raw(1),
                alife::core::units::ResourceAmount::new(0.5).unwrap()
            )
        ]
    );
    assert_eq!(
        copying.waste_outputs,
        vec![(
            ResourceTypeId::from_raw(4),
            alife::core::units::ResourceAmount::new(0.2).unwrap()
        )]
    );

    let recombination = accounting.recombination.expect("recombination accounting");
    assert_eq!(recombination.energy_cost.raw(), 4.0);
    assert_eq!(recombination.precursor_requirements.len(), 4);
}

#[test]
fn rejects_unknown_genome_precursor_resource_id() {
    let err = RawScenarioConfig::parse(&fixture().replace(
        "precursor_requirements = { nucleotide_precursor = 1.0, phosphate = 0.5 }",
        "precursor_requirements = { missing_resource = 1.0, phosphate = 0.5 }",
    ))
    .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("Unknown genome precursor resource"))
    );
}

#[test]
fn rejects_zero_genome_precursor_requirement() {
    let err = RawScenarioConfig::parse(&fixture().replace(
        "precursor_requirements = { nucleotide_precursor = 1.0, phosphate = 0.5 }",
        "precursor_requirements = { nucleotide_precursor = 0.0, phosphate = 0.5 }",
    ))
    .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("genome precursor requirement"))
    );
}

#[test]
fn genome_physical_accounting_participates_in_runtime_config_hash() {
    let baseline = RawScenarioConfig::parse(&fixture()).unwrap();
    let changed = RawScenarioConfig::parse(&fixture().replace(
        "precursor_requirements = { nucleotide_precursor = 1.0, phosphate = 0.5 }",
        "precursor_requirements = { nucleotide_precursor = 1.1, phosphate = 0.5 }",
    ))
    .unwrap();

    assert_ne!(baseline.config_hash(), changed.config_hash());
}

fn copying_candidate() -> ActionCandidate {
    ActionCandidate {
        process_id: ProcessId::GenomeCopying,
        requested_amount: 1.0,
    }
}

fn recombination_candidate() -> ActionCandidate {
    ActionCandidate {
        process_id: ProcessId::GenomeRecombination,
        requested_amount: 1.0,
    }
}

fn contact_pair_config() -> alife::core::config::RuntimeConfig {
    let mut config = RawScenarioConfig::parse(&fixture()).unwrap();
    let mut second = config.cell.clone();
    second.position = Position::new(17.5, 16.0);
    config.initial_cells = vec![config.cell.clone(), second];
    config.initial_typed_resources = vec![
        config.initial_typed_resources[0].clone(),
        config.initial_typed_resources[0].clone(),
    ];
    config.initial_cell_genome_templates = vec![
        config.initial_cell_genome_templates[0].clone(),
        config.initial_cell_genome_templates[0].clone(),
    ];
    config
}

#[test]
fn genome_copying_consumes_typed_precursors_and_not_generic_resource() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);

    assert!(matches!(
        executor
            .world()
            .validate_feasibility(cell, &copying_candidate()),
        FeasibilityResult::Allowed { .. }
    ));
    let before_generic = executor.world().cells().generic_resource_amount(cell);

    executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .unwrap();

    let cells = executor.world().cells();
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
            .unwrap()
            .raw(),
        2.0
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(1))
            .unwrap()
            .raw(),
        1.5
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(4))
            .unwrap()
            .raw(),
        0.2
    );
    assert_eq!(cells.generic_resource_amount(cell), before_generic);
    assert_eq!(cells.energy(cell).current().raw(), 7.5);
    assert_eq!(cells.genome_copy_progress(cell), 0.5);
    assert_eq!(cells.copied_genome_carrier_amount(cell), 0.5);
}

#[test]
fn genome_copying_missing_typed_precursor_rejects_without_state_change() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(cell, ResourceTypeId::from_raw(1), ResourceAmount::zero())
        .unwrap();

    let before_energy = executor.world().cells().energy(cell).current();
    let before_nucleotide = executor
        .world()
        .cells()
        .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
        .unwrap();

    assert_eq!(
        executor
            .world()
            .validate_feasibility(cell, &copying_candidate()),
        FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
    );
    assert!(executor
        .world_mut()
        .execute_genome_copying(cell, &copying_candidate())
        .is_err());

    let cells = executor.world().cells();
    assert_eq!(cells.energy(cell).current(), before_energy);
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
            .unwrap(),
        before_nucleotide
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(1))
            .unwrap(),
        ResourceAmount::zero()
    );
    assert_eq!(cells.genome_copy_progress(cell), 0.0);
    assert_eq!(cells.copied_genome_carrier_amount(cell), 0.0);
    assert!(cells.copied_genome_id(cell).is_none());
}

#[test]
fn genome_recombination_consumes_configured_precursors() {
    let mut executor = TickExecutor::new(contact_pair_config()).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);

    assert!(matches!(
        executor
            .world()
            .validate_feasibility(cell, &recombination_candidate()),
        FeasibilityResult::Allowed { .. }
    ));

    executor
        .world_mut()
        .execute_genome_recombination(cell, &recombination_candidate())
        .unwrap();

    let cells = executor.world().cells();
    assert_eq!(cells.energy(cell).current().raw(), 4.0);
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
            .unwrap()
            .raw(),
        2.4
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(1))
            .unwrap()
            .raw(),
        1.6
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(2))
            .unwrap()
            .raw(),
        0.6
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(3))
            .unwrap()
            .raw(),
        0.8
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(4))
            .unwrap()
            .raw(),
        0.3
    );
}

#[test]
fn genome_recombination_missing_precursor_rejects_without_state_change() {
    let mut executor = TickExecutor::new(contact_pair_config()).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(cell, ResourceTypeId::from_raw(3), ResourceAmount::zero())
        .unwrap();
    let before_energy = executor.world().cells().energy(cell).current();
    let before_nucleotide = executor
        .world()
        .cells()
        .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
        .unwrap();
    let before_genome = executor.world().cells().genome_id(cell);

    assert_eq!(
        executor
            .world()
            .validate_feasibility(cell, &recombination_candidate()),
        FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
    );
    assert!(executor
        .world_mut()
        .execute_genome_recombination(cell, &recombination_candidate())
        .is_err());

    let cells = executor.world().cells();
    assert_eq!(cells.energy(cell).current(), before_energy);
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
            .unwrap(),
        before_nucleotide
    );
    assert_eq!(
        cells
            .typed_resource_amount(cell, ResourceTypeId::from_raw(3))
            .unwrap(),
        ResourceAmount::zero()
    );
    assert_eq!(cells.genome_id(cell), before_genome);
}

#[test]
fn dead_cell_decomposition_moves_genome_carrier_to_fragment_accounting() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_lifecycle_state(cell, LifecycleState::Dead);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_boundary_material(cell, alife::core::units::MaterialAmount::zero());
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_synthesis_material(cell, alife::core::units::MaterialAmount::zero());
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_repair_material(cell, alife::core::units::MaterialAmount::zero());
    let before_grid_waste = executor
        .world()
        .resources()
        .amount_at(
            alife::core::resources::ResourceLayerIndex::from_raw(4),
            executor
                .world()
                .resources()
                .coord_for_position(executor.world().cells().position(cell)),
        )
        .unwrap();

    executor.world_mut().execute_decomposition_for_dead_cells();

    let cells = executor.world().cells();
    assert_eq!(cells.genome_carrier_amount(cell), 0.0);
    assert_eq!(executor.world().fragments().total_amount().raw(), 1.0);
    let after_grid_waste = executor
        .world()
        .resources()
        .amount_at(
            alife::core::resources::ResourceLayerIndex::from_raw(4),
            executor
                .world()
                .resources()
                .coord_for_position(executor.world().cells().position(cell)),
        )
        .unwrap();
    assert_eq!(after_grid_waste, before_grid_waste);
}
