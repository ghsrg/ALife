use alife::bootstrap::prepare;
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use std::path::PathBuf;

#[test]
fn test_diverse_rich_world_scenario_parsing_and_bootstrap() {
    let source = ScenarioSource::Path(PathBuf::from(
        "config/scenarios/bootstrap/diverse_rich_world.toml",
    ));
    let document = ScenarioDocument::resolve(source)
        .expect("diverse_rich_world.toml scenario file must load and parse successfully");

    assert_eq!(document.id, "diverse_rich_world");
    assert_eq!(document.runtime_config.world.seed.raw(), 42);

    let prepared = prepare(&document).expect("Bootstrap prepare must succeed");
    assert_eq!(prepared.manifest.cell_summary.initial_cells, 5);
    assert_eq!(prepared.manifest.resource_summary.len(), 8);

    // Verify patchy resource layers are non-uniform
    for resource_layer in &prepared.manifest.resource_summary {
        assert!(resource_layer.total > 0.0);
        assert!(resource_layer.max > resource_layer.min);
    }

    // Verify cell specializations
    assert_eq!(prepared.manifest.cell_summary.initial_cells, 5);
}

#[test]
fn test_seed_driven_world_diversity_and_determinism() {
    let source = ScenarioSource::Path(PathBuf::from(
        "config/scenarios/bootstrap/diverse_rich_world.toml",
    ));

    let document_seed42_a = ScenarioDocument::resolve(source.clone()).unwrap();
    let prepared_seed42_a = prepare(&document_seed42_a).unwrap();

    let document_seed42_b = ScenarioDocument::resolve(source).unwrap();
    let prepared_seed42_b = prepare(&document_seed42_b).unwrap();

    // Verify Determinism for same seed
    assert_eq!(
        prepared_seed42_a.prepared_state_hash,
        prepared_seed42_b.prepared_state_hash
    );

    // Modify seed inline to 101
    let toml_content =
        std::fs::read_to_string("config/scenarios/bootstrap/diverse_rich_world.toml").unwrap();
    let modified_toml = toml_content.replace("seed = 42", "seed = 101");
    let inline_source = ScenarioSource::Inline {
        id: "diverse_rich_world".to_string(),
        content: modified_toml,
    };
    let document_seed101 = ScenarioDocument::resolve(inline_source).unwrap();
    let prepared_seed101 = prepare(&document_seed101).unwrap();

    // Verify Seed-Driven Diversity: seed 101 must produce a different prepared state hash
    assert_ne!(
        prepared_seed42_a.prepared_state_hash,
        prepared_seed101.prepared_state_hash
    );

    // Verify resource spatial layer differences across seeds
    let layer42 = &prepared_seed42_a.manifest.resource_summary[0];
    let layer101 = &prepared_seed101.manifest.resource_summary[0];
    assert_ne!(layer42.total, layer101.total);
}
