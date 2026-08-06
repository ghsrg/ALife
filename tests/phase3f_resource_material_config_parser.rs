use alife::core::cell_store::CellIndex;
use alife::core::ids::ResourceTypeId;
use alife::core::world::WorldState;
use alife::runner::config_parser::{ParseError, RawScenarioConfig};

fn fixture() -> String {
    r#"
scenario_id = "phase3f"
seed = 7
tick_count = 10
[world]
size = [32.0, 32.0]
[space]
spatial_grid_size = 8.0
[resources]
resource_type_ids = ["amino_acid", "phospholipid", "inert_waste"]
initial_distribution = [10.0, 10.0, 0.0]
optional_decay_rate = 0.0
[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { amino_acid = 1.0, phospholipid = 3.0 }
initial_materials = { synthesis = 1.0 }
initial_energy = 6.0
energy_capacity = 10.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0
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
[synthesis]
cost_resource = 0.0
cost_energy = 4.0

[chemistry.resources.amino_acid]
volume = 1.0
diffusion_rate = 0.2
energy_value = 0.0
decay_rate = 0.01
reactivity_profile = "reactive"
permeability = "passive"
tags = ["structural_precursor", "dissolved"]
material_profile = { volume = 0.4, stability = 0.5, strength = 0.3, energy_capacity = 0.2, permeability = 0.7, durability = 0.4 }
material_capabilities = { material_synthesis = 0.8 }

[chemistry.resources.phospholipid]
volume = 1.0
diffusion_rate = 0.1
energy_value = 0.0
decay_rate = 0.01
reactivity_profile = "stable"
permeability = "passive"
tags = ["structural_precursor"]
material_profile = { volume = 0.8, stability = 0.7, strength = 0.6, energy_capacity = 0.1, permeability = 0.3, durability = 0.9 }
material_capabilities = { boundary_permeability = 0.6 }

[chemistry.resources.inert_waste]
volume = 1.0
diffusion_rate = 0.05
energy_value = 0.0
decay_rate = 0.02
reactivity_profile = "stable"
permeability = "blocked"
tags = ["waste"]
material_profile = { volume = 0.2, stability = 0.8, strength = 0.2, energy_capacity = 0.0, permeability = 0.1, durability = 0.8 }
material_capabilities = {}

[chemistry.reactions.flexible_membrane_synthesis]
mode = "controlled"
process_id = "material_synthesis"
inputs = { amino_acid = 1.0, phospholipid = 3.0 }
required_materials = {}
outputs = { inert_waste = 0.5 }
configured_sink_amount = 1.5
energy_output = 0.0
heat_output = 0.2
rate = 1.0
probability = 1.0
accounting_destination = "inert_waste"
material_output = { amount = 2.0, derivation = "volume_weighted" }

[chemistry.heat]
capacity = 10.0
dissipation_rate = 0.2
warning_threshold = 8.0
death_threshold = 10.0
[chemistry.boundary]
default_permeability = "blocked"
retention_rate = 0.9
[chemistry.repair]
enabled = false
energy_cost = 0.0
max_amount_per_tick = 0.0
"#
    .to_string()
}

#[test]
fn parses_resource_material_profiles_and_material_output_reactions() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();

    let amino = &config.chemistry.resources[0];
    let profile = amino.material_profile.unwrap();
    assert_eq!(profile.strength(), 0.3);
    assert_eq!(
        amino
            .material_capabilities
            .value(alife::core::process::MaterialCapability::MaterialSynthesis),
        0.8
    );

    let reaction = &config.chemistry.reactions[0];
    let material_output = reaction.material_output.as_ref().unwrap();
    assert_eq!(reaction.process_id.as_deref(), Some("material_synthesis"));
    assert_eq!(material_output.amount.raw(), 2.0);
    assert_eq!(material_output.derivation, "volume_weighted");
}

#[test]
fn rejects_resource_material_profile_values_outside_unit_bounds() {
    let err = RawScenarioConfig::parse(&fixture().replace("strength = 0.3", "strength = 1.3"))
        .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("material_profile"))
    );
}

#[test]
fn rejects_unknown_material_capability_keys() {
    let err = RawScenarioConfig::parse(
        &fixture().replace("material_synthesis = 0.8", "photosynthesis = 0.8"),
    )
    .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("Unknown material capability"))
    );
}

#[test]
fn world_execute_synthesis_creates_resource_derived_material_instance() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();
    let mut world = WorldState::from_config(config).unwrap();
    let cell = CellIndex::from_raw(0);

    world.execute_synthesis(cell).unwrap();

    assert_eq!(world.cells().energy(cell).current().raw(), 2.0);
    assert_eq!(
        world
            .cells()
            .typed_resource_amount(cell, ResourceTypeId::from_raw(0))
            .unwrap()
            .raw(),
        0.0
    );
    assert_eq!(
        world
            .cells()
            .typed_resource_amount(cell, ResourceTypeId::from_raw(1))
            .unwrap()
            .raw(),
        0.0
    );
    assert_eq!(
        world
            .cells()
            .typed_resource_amount(cell, ResourceTypeId::from_raw(2))
            .unwrap()
            .raw(),
        0.5
    );
    assert_eq!(world.environment().heat().raw(), 0.2);
    assert_eq!(world.cells().synthesis_material(cell).raw(), 1.0);
    assert_eq!(world.cells().material_instances(cell).len(), 1);
    assert!(
        (world.cells().material_instances(cell)[0]
            .profile()
            .strength()
            - 0.525)
            .abs()
            < 0.000_001
    );
}

#[test]
fn material_profile_and_output_participate_in_runtime_config_hash() {
    let baseline = RawScenarioConfig::parse(&fixture()).unwrap();
    let changed_profile =
        RawScenarioConfig::parse(&fixture().replace("strength = 0.3", "strength = 0.31")).unwrap();
    let changed_output_source = fixture()
        .replace(
            "configured_sink_amount = 1.5",
            "configured_sink_amount = 1.6",
        )
        .replace(
            "material_output = { amount = 2.0",
            "material_output = { amount = 1.9",
        );
    let changed_output = RawScenarioConfig::parse(&changed_output_source).unwrap();

    assert_ne!(baseline.config_hash(), changed_profile.config_hash());
    assert_ne!(baseline.config_hash(), changed_output.config_hash());
}
