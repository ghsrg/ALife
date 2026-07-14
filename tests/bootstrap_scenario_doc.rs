use alife::runner::scenario_doc::{
    ScenarioDocument, ScenarioSource, canonicalize_scenario_source_v1, scenario_hash_v1,
};

const MINIMAL_SCENARIO: &str = r#"
scenario_id = "bootstrap_doc_test"
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

#[test]
fn same_inline_content_gets_same_hash_independent_of_source_id() {
    let a = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "path_a/bootstrap_doc_test.toml".to_string(),
        content: MINIMAL_SCENARIO.to_string(),
    })
    .unwrap();
    let b = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "path_b/bootstrap_doc_test.toml".to_string(),
        content: MINIMAL_SCENARIO.to_string(),
    })
    .unwrap();

    assert_eq!(a.scenario_hash, b.scenario_hash);
    assert_eq!(
        a.runtime_config.config_hash(),
        b.runtime_config.config_hash()
    );
}

#[test]
fn seed_is_part_of_canonical_hash() {
    let a = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "seed_a".to_string(),
        content: MINIMAL_SCENARIO.to_string(),
    })
    .unwrap();
    let b = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "seed_b".to_string(),
        content: MINIMAL_SCENARIO.replace("seed = 42", "seed = 43"),
    })
    .unwrap();

    assert_ne!(a.scenario_hash, b.scenario_hash);
}

#[test]
fn canonical_hash_is_stable_for_supported_whitespace_changes() {
    let compact = r#"scenario_id="bootstrap_doc_test"
seed=42
tick_count=10
"#;
    let spaced = r#"
scenario_id = "bootstrap_doc_test"

seed = 42
tick_count = 10
"#;

    assert_eq!(
        scenario_hash_v1(&canonicalize_scenario_source_v1(compact)),
        scenario_hash_v1(&canonicalize_scenario_source_v1(spaced))
    );
}

#[test]
fn scenario_document_preserves_runtime_config_for_core_start() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "runtime".to_string(),
        content: MINIMAL_SCENARIO.to_string(),
    })
    .unwrap();

    assert_eq!(document.schema_version, 1);
    assert_eq!(document.runtime_config.world.seed.raw(), 42);
    assert_eq!(document.runtime_config.initial_cells.len(), 1);
}

#[test]
fn invalid_toml_returns_typed_resolution_error() {
    let err = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "broken".to_string(),
        content: "scenario_id = ".to_string(),
    })
    .unwrap_err();

    assert_eq!(err.code(), "SCENARIO_PARSE_FAILED");
}
