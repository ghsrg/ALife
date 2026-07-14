use alife::bootstrap::{BootstrapError, prepare};
use alife::core::tick::TickExecutor;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

const SCENARIO: &str = r#"
scenario_id = "bootstrap_prepare_test"
seed = 42
tick_count = 10

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [20.0]
optional_decay_rate = 0.0

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

fn document() -> ScenarioDocument {
    ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "bootstrap_prepare_test".to_string(),
        content: SCENARIO.to_string(),
    })
    .unwrap()
}

#[test]
fn prepare_returns_prepared_world_with_matching_manifest_hash() {
    let document = document();
    let prepared = prepare(&document).unwrap();

    assert_eq!(prepared.manifest.scenario_hash, document.scenario_hash);
    assert_eq!(
        prepared.manifest.prepared_state_hash,
        prepared.prepared_state_hash
    );
    assert_eq!(prepared.manifest.root_seed, 42);
}

#[test]
fn prepared_world_runtime_config_can_construct_core_without_tick() {
    let prepared = prepare(&document()).unwrap();
    let executor = TickExecutor::new(prepared.runtime_config).unwrap();

    assert_eq!(executor.world().tick().raw(), 0);
}

#[test]
fn prepare_is_deterministic_for_same_document() {
    let document = document();
    let a = prepare(&document).unwrap();
    let b = prepare(&document).unwrap();

    assert_eq!(a.prepared_state_hash, b.prepared_state_hash);
    assert_eq!(a.manifest.generator_versions, b.manifest.generator_versions);
    assert_eq!(a.manifest.seed_domains, b.manifest.seed_domains);
}

#[test]
fn manifest_contains_minimum_generator_versions_and_summaries() {
    let prepared = prepare(&document()).unwrap();
    let versions: Vec<_> = prepared
        .manifest
        .generator_versions
        .iter()
        .map(|version| version.name.as_str())
        .collect();

    assert!(versions.contains(&"seed_domains.v1"));
    assert!(versions.contains(&"prepared_world.v1"));
    assert_eq!(prepared.manifest.world_summary.initial_cells, 1);
    assert_eq!(prepared.manifest.cell_summary.initial_cells, 1);
}

#[test]
fn structurally_impossible_world_returns_typed_error() {
    let mut document = document();
    document.runtime_config.initial_cells.clear();

    let err = prepare(&document).unwrap_err();
    assert!(matches!(err, BootstrapError::Viability(_)));
    assert_eq!(err.code(), "BOOTSTRAP_VIABILITY_FAILED");
}
