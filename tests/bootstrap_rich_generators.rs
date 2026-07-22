use alife::bootstrap::prepare;
use alife::core::resources::ResourceLayerIndex;
use alife::core::world::WorldState;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

const RICH_GENERATOR_SCENARIO: &str = r#"
scenario_id = "bootstrap_rich_generator_doc_test"
seed = 42
tick_count = 10
legacy_material_distribution = false

[world]
size = [64.0, 64.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A", "waste_A"]
initial_distribution = [0.0, 0.0]
optional_decay_rate = 0.0

[bootstrap]
family = "patchy_temperate_v1"

[[bootstrap.resources]]
resource_type_id = "nutrient_A"
generator = "patches"
version = "patches.v1"
seed_domain = "resources.layers.nutrient_A"
patches = 3
min_amount = 1.0
max_amount = 4.0
falloff = 0.5

[[bootstrap.fields]]
field_id = "temperature"
generator = "band"
version = "band.v1"
seed_domain = "fields.layers.temperature"
min_value = 18.0
max_value = 27.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 8.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.1
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = false
critical_capacity_overrun = 5.0
"#;

#[test]
fn scenario_document_preserves_typed_bootstrap_generator_spec() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "rich_generator".to_string(),
        content: RICH_GENERATOR_SCENARIO.to_string(),
    })
    .unwrap();

    let spec = document.bootstrap_spec.as_ref().expect("bootstrap spec");
    assert_eq!(spec.family.as_deref(), Some("patchy_temperate_v1"));
    assert_eq!(spec.resources.len(), 1);
    assert_eq!(spec.resources[0].resource_type_id, "nutrient_A");
    assert_eq!(spec.resources[0].generator, "patches");
    assert_eq!(spec.resources[0].version, "patches.v1");
    assert_eq!(spec.resources[0].seed_domain, "resources.layers.nutrient_A");
    assert_eq!(spec.resources[0].patches, Some(3));
    assert_eq!(spec.resources[0].min_amount, Some(1.0));
    assert_eq!(spec.resources[0].max_amount, Some(4.0));
    assert_eq!(spec.resources[0].falloff, Some(0.5));
    assert_eq!(spec.fields.len(), 1);
    assert_eq!(spec.fields[0].field_id, "temperature");
    assert_eq!(spec.fields[0].generator, "band");
    assert_eq!(spec.fields[0].version, "band.v1");
    assert_eq!(spec.fields[0].seed_domain, "fields.layers.temperature");
}

#[test]
fn rich_bootstrap_prepares_deterministic_spatial_resource_grid() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "rich_generator".to_string(),
        content: RICH_GENERATOR_SCENARIO.to_string(),
    })
    .unwrap();

    let prepared_a = prepare(&document).unwrap();
    let prepared_b = prepare(&document).unwrap();

    assert_eq!(
        prepared_a.runtime_config.prepared_resource_layers,
        prepared_b.runtime_config.prepared_resource_layers
    );
    let layers = prepared_a
        .runtime_config
        .prepared_resource_layers
        .as_ref()
        .expect("prepared resource layers");
    assert_eq!(layers.len(), 2);
    assert_eq!(layers[0].len(), 64);
    assert!(layers[0].iter().any(|amount| amount.raw() > 1.0));
    assert!(layers[0].iter().any(|amount| amount.raw() < 4.0));
    assert!(layers[1].iter().all(|amount| amount.raw() == 0.0));

    let summary = prepared_a
        .manifest
        .resource_summary
        .iter()
        .find(|summary| summary.layer_index == 0)
        .unwrap();
    assert!(summary.max > summary.min);
    assert!(
        prepared_a
            .manifest
            .seed_domains
            .iter()
            .any(|record| record.label == "resources.layers.nutrient_A")
    );
    assert_eq!(
        prepared_a.manifest.world_family.as_ref().unwrap().family_id,
        "patchy_temperate_v1"
    );
    assert!(
        prepared_a
            .manifest
            .warnings
            .iter()
            .any(|warning| { warning.code == "BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED" })
    );

    let world = WorldState::from_config(prepared_a.runtime_config).unwrap();
    let total = world
        .resources()
        .total_amount_for_layer(ResourceLayerIndex::from_raw(0))
        .unwrap()
        .raw();
    assert!((total - summary.total).abs() < 0.001);
}
