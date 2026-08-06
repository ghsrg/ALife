use alife::core::fields::{
    FieldConservedBehavior, FieldEffectProfile, FieldKind, FieldRuntimeConfig,
};
use alife::core::fields::{FieldGrid, FieldGridError, FieldLayerIndex};
use alife::core::ids::FieldTypeId;
use alife::core::stable_state_hash::StableStateHasher;
use alife::core::tick::TickExecutor;
use alife::core::units::{FieldValue, GridCoord, MaterialAmount, Position, WorldSize};
use alife::runner::config_parser::{ParseError, RawScenarioConfig};
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

fn minimal_field_toml(extra_fields: &str, extra_reaction: &str) -> String {
    format!(
        r#"
scenario_id = "field_test"
seed = 11
tick_count = 20

[world]
size = [20.0, 20.0]

[space]
spatial_grid_size = 10.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["fuel", "waste"]
initial_distribution = [10.0, 0.0]
optional_decay_rate = 0.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 80.0
heat_death_threshold = 160.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 80.0
waste_death_threshold = 160.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 100.0
dormant_mandatory_cost_modifier = 1.0

[cell]
initial_position = [5.0, 5.0]
radius = 1.0
initial_energy = 10.0
energy_capacity = 20.0
initial_resources = {{ fuel = 5.0, waste = 0.0 }}
initial_materials = {{ synthesis = 1.0 }}
mandatory_cost_per_tick = 0.0
capacity_limit = 100.0

[scheduler.world]
field_update_ticks = 5

{extra_fields}

[chemistry.resources.fuel]
volume = 1.0
diffusion_rate = 0.0
energy_value = 1.0
decay_rate = 0.0
reactivity_profile = "reactive"
permeability = "passive"
tags = []

[chemistry.resources.waste]
volume = 1.0
diffusion_rate = 0.0
energy_value = 0.0
decay_rate = 0.0
reactivity_profile = "stable"
permeability = "blocked"
tags = []

{extra_reaction}
"#
    )
}

fn temperature_field_config() -> &'static str {
    r#"
[fields.temperature]
kind = "scalar"
initial_value = 25.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 100.0
effect_profile = "temperature"
conserved_behavior = "abstracted"
"#
}

#[test]
fn parses_bounded_scalar_field_config_and_hashes_it() {
    let base = RawScenarioConfig::parse(&minimal_field_toml(temperature_field_config(), ""))
        .expect("valid field config should parse");
    assert_eq!(base.fields.len(), 1);
    assert_eq!(base.fields[0].id, "temperature");
    assert_eq!(base.fields[0].type_id, FieldTypeId::from_raw(0));
    assert_eq!(base.fields[0].initial_value, FieldValue::new(25.0).unwrap());

    let changed = RawScenarioConfig::parse(&minimal_field_toml(
        &temperature_field_config().replace("max_value = 100.0", "max_value = 90.0"),
        "",
    ))
    .expect("changed bounded field config should parse");
    assert_ne!(base.config_hash(), changed.config_hash());
}

#[test]
fn rejects_invalid_field_config() {
    for fields in [
        temperature_field_config().replace("min_value = 0.0", "min_value = 101.0"),
        temperature_field_config().replace("initial_value = 25.0", "initial_value = 101.0"),
        temperature_field_config().replace("diffusion_rate = 0.0", "diffusion_rate = -0.1"),
        temperature_field_config().replace("decay_rate = 0.0", "decay_rate = -0.1"),
        temperature_field_config().replace(
            "effect_profile = \"temperature\"",
            "effect_profile = \"command\"",
        ),
    ] {
        assert!(matches!(
            RawScenarioConfig::parse(&minimal_field_toml(&fields, "")),
            Err(ParseError::ValidationError(_))
        ));
    }
}

#[test]
fn field_grid_samples_local_values_and_clamps_updates() {
    let field_id = FieldTypeId::from_raw(0);
    let mut grid = FieldGrid::new(
        alife::core::units::WorldSize::new(20.0, 20.0).unwrap(),
        10.0,
        vec![(field_id, FieldValue::new(25.0).unwrap())],
        0.0,
        100.0,
        0.0,
        0.0,
    )
    .unwrap();

    grid.set_value_at(
        FieldLayerIndex::from_raw(0),
        GridCoord::new(1, 0),
        FieldValue::new(200.0).unwrap(),
    )
    .unwrap();

    assert_eq!(
        grid.sample_at_position(field_id, Position::new(5.0, 5.0)),
        Ok(FieldValue::new(25.0).unwrap())
    );
    assert_eq!(
        grid.sample_at_position(field_id, Position::new(15.0, 5.0)),
        Ok(FieldValue::new(100.0).unwrap())
    );
    assert_eq!(
        grid.sample_at_position(FieldTypeId::from_raw(99), Position::new(5.0, 5.0)),
        Err(FieldGridError::LayerOutOfBounds)
    );
}

#[test]
fn field_grid_preserves_per_layer_bounds_from_runtime_configs() {
    let configs = vec![
        FieldRuntimeConfig::new(
            "temperature".to_string(),
            FieldTypeId::from_raw(0),
            FieldKind::Scalar,
            FieldValue::new(5.0).unwrap(),
            0.0,
            0.0,
            FieldValue::new(0.0).unwrap(),
            FieldValue::new(10.0).unwrap(),
            FieldEffectProfile::Temperature,
            FieldConservedBehavior::Abstracted,
        )
        .unwrap(),
        FieldRuntimeConfig::new(
            "light".to_string(),
            FieldTypeId::from_raw(1),
            FieldKind::Scalar,
            FieldValue::new(20.0).unwrap(),
            0.0,
            0.0,
            FieldValue::new(0.0).unwrap(),
            FieldValue::new(100.0).unwrap(),
            FieldEffectProfile::Light,
            FieldConservedBehavior::Abstracted,
        )
        .unwrap(),
    ];
    let mut grid = FieldGrid::from_configs(WorldSize::new(10.0, 10.0).unwrap(), 10.0, &configs)
        .unwrap()
        .unwrap();
    let coord = GridCoord::new(0, 0);

    grid.set_value_at(
        FieldLayerIndex::from_raw(0),
        coord,
        FieldValue::new(50.0).unwrap(),
    )
    .unwrap();
    grid.set_value_at(
        FieldLayerIndex::from_raw(1),
        coord,
        FieldValue::new(50.0).unwrap(),
    )
    .unwrap();

    assert_eq!(
        grid.value_at(FieldLayerIndex::from_raw(0), coord).unwrap(),
        FieldValue::new(10.0).unwrap()
    );
    assert_eq!(
        grid.value_at(FieldLayerIndex::from_raw(1), coord).unwrap(),
        FieldValue::new(50.0).unwrap()
    );
}

#[test]
fn world_field_runtime_is_read_only_without_registered_effects() {
    let config = RawScenarioConfig::parse(&minimal_field_toml(temperature_field_config(), ""))
        .expect("valid field config should parse");
    let mut executor = TickExecutor::new(config).unwrap();
    let before_energy = executor
        .world()
        .cells()
        .energy(alife::core::cell_store::CellIndex::from_raw(0));
    let before_hash = StableStateHasher::hash_world(executor.world());

    executor.step().unwrap();

    assert_eq!(
        executor
            .world()
            .local_field_sample(
                alife::core::cell_store::CellIndex::from_raw(0),
                FieldTypeId::from_raw(0)
            )
            .unwrap(),
        FieldValue::new(25.0).unwrap()
    );
    assert_eq!(
        executor
            .world()
            .cells()
            .energy(alife::core::cell_store::CellIndex::from_raw(0)),
        before_energy
    );
    assert_ne!(before_hash, StableStateHasher::hash_world(executor.world()));
}

#[test]
fn scheduled_field_decay_integrates_elapsed_ticks() {
    let config = RawScenarioConfig::parse(&minimal_field_toml(
        &temperature_field_config()
            .replace("initial_value = 25.0", "initial_value = 80.0")
            .replace("decay_rate = 0.0", "decay_rate = 0.1"),
        "",
    ))
    .expect("valid field config should parse");
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);

    for _ in 0..4 {
        executor.step().unwrap();
    }
    assert_eq!(
        executor
            .world()
            .local_field_sample(cell, FieldTypeId::from_raw(0))
            .unwrap(),
        FieldValue::new(80.0).unwrap()
    );

    executor.step().unwrap();
    let decayed = executor
        .world()
        .local_field_sample(cell, FieldTypeId::from_raw(0))
        .unwrap();
    assert!((decayed.raw() - 47.239197).abs() < 0.0001);
}

#[test]
fn field_condition_gates_material_synthesis_without_direct_energy_or_genome_effects() {
    let reaction = r#"
[chemistry.reactions.cool_synthesis]
mode = "controlled"
process_id = "material_synthesis"
inputs = { fuel = 1.0 }
required_materials = {}
outputs = { waste = 0.5 }
configured_sink_amount = 0.0
energy_output = 0.0
heat_output = 0.0
rate = 1.0
probability = 1.0
accounting_destination = "waste"
material_output = { amount = 0.5, derivation = "volume_weighted" }
field_condition = { field_id = "temperature", min = 0.0, max = 30.0 }

[chemistry.resources.fuel.material_profile]
volume = 1.0
stability = 0.5
strength = 0.5
energy_capacity = 0.0
permeability = 0.5
durability = 0.5
"#;
    let mut config = RawScenarioConfig::parse(&minimal_field_toml(
        &temperature_field_config().replace("initial_value = 25.0", "initial_value = 40.0"),
        reaction,
    ))
    .expect("valid field-gated reaction should parse");
    config.synthesis.cost_energy = alife::core::units::EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    let before_energy = executor.world().cells().energy(cell);
    let before_fuel = executor
        .world()
        .cells()
        .typed_resource_amount(cell, alife::core::ids::ResourceTypeId::from_raw(0))
        .unwrap();

    assert!(executor.world_mut().execute_synthesis(cell).is_err());

    assert_eq!(executor.world().cells().energy(cell), before_energy);
    assert_eq!(
        executor
            .world()
            .cells()
            .typed_resource_amount(cell, alife::core::ids::ResourceTypeId::from_raw(0))
            .unwrap(),
        before_fuel
    );
    assert_eq!(executor.world().cells().material_instances(cell).len(), 0);
}

#[test]
fn field_condition_scales_configured_material_degradation_without_energy_or_genome_effects() {
    let materials = r#"
[chemistry.materials.boundary_polymer_A]
volume = 1.0
stability = 0.8
strength = 0.7
permeability = 0.1
energy_capacity = 0.0
decay_rate = 0.1
repair_resource = "fuel"
repair_amount = 0.25

[chemistry.materials.boundary_polymer_A.field_degradation]
field_id = "temperature"
min = 50.0
max = 100.0
multiplier = 2.0
"#;
    let toml = minimal_field_toml(
        &temperature_field_config().replace("initial_value = 25.0", "initial_value = 80.0"),
        materials,
    )
    .replace(
        "initial_materials = { synthesis = 1.0 }",
        "initial_materials = { boundary = 1.0, synthesis = 1.0 }",
    );
    let config = RawScenarioConfig::parse(&toml).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();
    let before_hash = StableStateHasher::hash_world(executor.world());
    let before_energy = executor
        .world()
        .cells()
        .energy(alife::core::cell_store::CellIndex::from_raw(0))
        .current();

    let summary = executor.step().unwrap();

    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    assert_eq!(
        executor.world().cells().boundary_material(cell),
        MaterialAmount::new(0.8).unwrap()
    );
    assert_eq!(summary.metrics.material_degradation_amount, 0.2);
    assert_eq!(
        executor.world().cells().energy(cell).current(),
        before_energy,
        "field-mediated material degradation must not credit or debit energy directly"
    );
    assert_ne!(
        before_hash,
        StableStateHasher::hash_world(executor.world()),
        "committed material degradation remains visible to stable state hash"
    );
}

#[test]
fn bootstrap_field_spec_initializes_matching_core_field_layer() {
    let fields = r#"
[[bootstrap.fields]]
field_id = "temperature"
generator = "band"
version = "band.v1"
seed_domain = "fields.layers.temperature"
min_value = 40.0
max_value = 60.0

[fields.temperature]
kind = "scalar"
initial_value = 10.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 100.0
effect_profile = "temperature"
conserved_behavior = "abstracted"
"#;
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "bootstrap_field_runtime".to_string(),
        content: minimal_field_toml(fields, ""),
    })
    .unwrap();
    let prepared = alife::bootstrap::prepare(&document).unwrap();
    let executor = TickExecutor::new(prepared.runtime_config).unwrap();

    let value = executor
        .world()
        .local_field_sample(
            alife::core::cell_store::CellIndex::from_raw(0),
            FieldTypeId::from_raw(0),
        )
        .unwrap();
    assert_eq!(value, FieldValue::new(50.0).unwrap());
    assert!(prepared.manifest.warnings.is_empty());
}
